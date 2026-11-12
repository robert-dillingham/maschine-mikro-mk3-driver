#!/bin/bash
# Connect the Maschine driver's MIDI port to REAPER's virtual MIDI port.
# Run it after the driver and REAPER are both running.

# client number of the driver (the line starts with "client 128: 'Maschine Mikro MK3'")
SRC=$(aconnect -l | awk '/^client/ && /Maschine Mikro MK3/ {gsub(":","",$2); print $2; exit}')

# client number that owns the "Virtual RawMIDI" port (REAPER's)
DST=$(aconnect -l | awk '/^client/ {c=$2; gsub(":","",c)} /Virtual RawMIDI/ {print c; exit}')

if [ -z "$SRC" ]; then echo "Driver not found. Is it running?"; exit 1; fi
if [ -z "$DST" ]; then echo "REAPER's virtual port not found. Is REAPER open with the device enabled?"; exit 1; fi

aconnect "$SRC:0" "$DST:0" && echo "Connected $SRC:0 -> $DST:0"