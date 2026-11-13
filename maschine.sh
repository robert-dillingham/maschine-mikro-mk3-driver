#!/bin/bash
# maschine.sh: start the Maschine Mikro MK3 driver and link it to the
# "Maschine" virtual MIDI device, so it shows up in REAPER as hw:Maschine.
#
# Usage:   ./maschine.sh              driver output goes to a log file
#          ./maschine.sh --verbose    driver output is shown in this terminal
#
# Put this file in the project folder (next to Cargo.toml) and run
#   chmod +x maschine.sh
# Stop everything with Ctrl+C.

set -u

CARD_ID="Maschine"                 # name of the virtual MIDI device (snd-virmidi id=...)
DRIVER_CLIENT="Maschine Mikro MK3" # client name the driver creates (client_name in settings)
CONFIG="example_config.toml"
BIN="target/release/driver"
LOG="/tmp/maschine-driver.log"
VERBOSE=0
DRIVER_PID=""

ok()   { echo "[ok]    $*"; }
info() { echo "[..]    $*"; }
warn() { echo "[warn]  $*"; }
fail() { echo "[error] $*" >&2; exit 1; }

# card number of the virtual MIDI device with our id (empty if none)
find_card() {
    grep -E "^ *[0-9]+ \[$CARD_ID *\]" /proc/asound/cards 2>/dev/null | awk '{print $1}' | head -n1
}

# ALSA client number belonging to that card (empty if none). $1 = card number
find_dst() {
    aconnect -l | awk -v card="$1" '
        /^client/ {
            c = $2; sub(":", "", c)
            if ($0 ~ ("card=" card "\\]")) { print c; exit }
        }'
}

# ALSA client number of the running driver (empty if not running)
find_src() {
    aconnect -l | awk -v name="$DRIVER_CLIENT" '
        /^client/ && index($0, name) { c = $2; sub(":", "", c); print c; exit }'
}

# is the driver's client ($1) connected to the device's client ($2)?
is_linked() {
    aconnect -l | awk -v s="$1" -v d="$2" '
        /^client/ { c = $2; sub(":", "", c); insrc = (c == s) }
        insrc && /Connecting To:/ {
            if ($0 ~ ("(^|[ ,])" d ":0([ ,]|$)")) found = 1
        }
        END { exit !found }'
}

cleanup() {
    if [ -n "$DRIVER_PID" ] && kill -0 "$DRIVER_PID" 2>/dev/null; then
        kill "$DRIVER_PID" 2>/dev/null
        wait "$DRIVER_PID" 2>/dev/null
        echo
        info "Driver stopped."
    fi
}

show_log_tail() {
    if [ "$VERBOSE" -eq 0 ] && [ -f "$LOG" ]; then
        echo "        Last lines of the driver log ($LOG):"
        tail -n 8 "$LOG" | sed 's/^/        | /'
    fi
}

main() {
    [ "${1:-}" = "--verbose" ] && VERBOSE=1

    cd "$(dirname "$(readlink -f "$0")")" || fail "Cannot find the project folder."

    # 1. tools
    command -v aconnect >/dev/null || fail "'aconnect' not found. Install it (package: alsa-utils)."
    command -v cargo    >/dev/null || fail "'cargo' not found. Install Rust (https://rustup.rs)."
    ok "Tools found."

    # 2. the virtual MIDI device
    CARD=$(find_card)
    if [ -z "$CARD" ]; then
        echo "[error] No virtual MIDI device named '$CARD_ID' found." >&2
        if lsmod | grep -q '^snd_virmidi'; then
            echo "        The snd-virmidi module is loaded, but without the name '$CARD_ID'." >&2
            echo "        Close REAPER, then run:" >&2
            echo "          sudo modprobe -r snd-virmidi" >&2
            echo "          sudo modprobe snd-virmidi midi_devs=1 id=$CARD_ID" >&2
            exit 1
        fi
        read -r -p "        Load it now with sudo? [y/N] " answer
        case "$answer" in
            y|Y)
                sudo modprobe snd-virmidi midi_devs=1 id="$CARD_ID" || fail "Could not load snd-virmidi."
                sleep 1
                CARD=$(find_card)
                [ -n "$CARD" ] || fail "Loaded snd-virmidi, but no device named '$CARD_ID' appeared."
                ok "Created the '$CARD_ID' virtual MIDI device (card $CARD)."
                warn "If REAPER is already open, click 'Reset all MIDI devices' in its MIDI Devices preferences."
                ;;
            *) exit 1 ;;
        esac
    else
        ok "Found virtual MIDI device '$CARD_ID' (card $CARD)."
    fi

    DST=$(find_dst "$CARD")
    [ -n "$DST" ] || fail "Card $CARD exists but has no MIDI sequencer client. Try unloading and reloading snd-virmidi."
    ok "Its MIDI port is client $DST."

    # 3. make sure the driver is not already running
    EXISTING=$(find_src)
    [ -z "$EXISTING" ] || fail "A '$DRIVER_CLIENT' port already exists (client $EXISTING). Is the driver already running? Stop it first."

    # 4. build (only recompiles if something changed)
    info "Building the driver..."
    cargo build --release --quiet || fail "The build failed. Fix the errors shown above."
    [ -x "$BIN" ] || fail "Built, but '$BIN' was not found. Check the binary name in Cargo.toml."
    ok "Driver is built."

    # 5. start the driver
    info "Starting the driver..."
    if [ "$VERBOSE" -eq 1 ]; then
        "$BIN" --config "$CONFIG" &
    else
        "$BIN" --config "$CONFIG" >"$LOG" 2>&1 &
    fi
    DRIVER_PID=$!
    trap cleanup EXIT
    trap 'exit 130' INT TERM

    # 6. wait for its MIDI port (up to 10 seconds)
    SRC=""
    for _ in $(seq 1 20); do
        if ! kill -0 "$DRIVER_PID" 2>/dev/null; then
            echo "[error] The driver stopped right after starting." >&2
            echo "        Is the Maschine plugged in, and is the udev rule installed?" >&2
            show_log_tail
            DRIVER_PID=""
            exit 1
        fi
        SRC=$(find_src)
        [ -n "$SRC" ] && break
        sleep 0.5
    done
    [ -n "$SRC" ] || fail "The driver is running but its MIDI port did not appear."
    ok "Driver is running (MIDI client $SRC)."

    # 7. link the driver to the virtual device
    aconnect "$SRC:0" "$DST:0" || fail "Could not link $SRC:0 to $DST:0."
    is_linked "$SRC" "$DST" || fail "The link was made but could not be confirmed."
    ok "Linked: $DRIVER_CLIENT -> hw:$CARD_ID"

    echo
    echo "Ready. In REAPER: set a track's input to  MIDI > hw:$CARD_ID > All Channels."
    echo "Press Ctrl+C to stop the driver."
    echo

    # 8. keep running; re-link if the link ever disappears
    while kill -0 "$DRIVER_PID" 2>/dev/null; do
        sleep 2
        NEW_DST=$(find_dst "$CARD")
        [ -n "$NEW_DST" ] && DST="$NEW_DST"
        SRC=$(find_src)
        if [ -n "$SRC" ] && ! is_linked "$SRC" "$DST"; then
            if aconnect "$SRC:0" "$DST:0" 2>/dev/null; then
                warn "The link was lost; re-linked to client $DST."
            fi
        fi
    done

    echo "[error] The driver stopped." >&2
    show_log_tail
    DRIVER_PID=""
    exit 1
}

# run main only when executed, not when sourced (so the functions can be tested)
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    main "$@"
fi
