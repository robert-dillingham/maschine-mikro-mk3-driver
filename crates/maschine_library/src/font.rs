use crate::screen::Screen;

const DIGITS: [[&[u8; 8]; 8]; 10] = [
    // 0
    [
        b"   xxx  ",
        b"  x   x ",
        b" x     x",
        b" x     x",
        b" x     x",
        b" x     x",
        b"  x   x ",
        b"   xxx  ",
    ],
    [
        // 1
        b"     xx ",
        b"     xx ",
        b"    x x ",
        b"  xx  x ",
        b"      x ",
        b"      x ",
        b"      x ",
        b"  xxxxxx",
    ],
    [
        // 2
        b"   xxxx ",
        b" x     x",
        b" x     x",
        b"      x ",
        b"    x   ",
        b"  x     ",
        b" x      ",
        b" xxxxxxx",
    ],
    [
        // 3
        b"  xxxxx ",
        b" x     x",
        b"      x ",
        b"   xxxx ",
        b"       x",
        b"       x",
        b" x    x ",
        b"  xxxx  ",
    ],
    [
        // 4
        b" x     x",
        b" x     x",
        b" x     x",
        b" x    xx",
        b"  xxxx x",
        b"       x",
        b"       x",
        b"       x",
    ],
    [
        // 5
        b" xxxxxxx",
        b" x      ",
        b" x      ",
        b" xxxxxx ",
        b"       x",
        b"       x",
        b"       x",
        b" xxxxxx ",
    ],
    [
        // 6
        b"  xxxxx ",
        b" x     x",
        b" x      ",
        b" x xxx  ",
        b" xx   xx",
        b" x     x",
        b" x     x",
        b"  xxxxx ",
    ],
    [
        // 7
        b" xxxxxxx",
        b"       x",
        b"       x",
        b"      x ",
        b"     x  ",
        b"    x   ",
        b"   x    ",
        b"  x     ",
    ],
    [
        // 8
        b"  xxxxx ",
        b" x     x",
        b" x     x",
        b"  xxxxx ",
        b" x     x",
        b" x     x",
        b" x     x",
        b"  xxxxx ",
    ],
    [
        // 9
        b"  xxxxx ",
        b" x     x",
        b" x     x",
        b" x     x",
        b"  xxxxxx",
        b"       x",
        b" x     x",
        b"  xxxxx ",
    ],
];

const LETTERS: [[&[u8; 8]; 8]; 26] = [
    // A
    [
        b"   xx   ",
        b"  x  x  ",
        b" x    x ",
        b" x    x ",
        b" xxxxxx ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
    ],
    // B
    [
        b" xxxxx  ",
        b" x    x ",
        b" x    x ",
        b" xxxxx  ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
        b" xxxxx  ",
    ],
    // C
    [
        b"  xxxx  ",
        b" x    x ",
        b"x       ",
        b"x       ",
        b"x       ",
        b"x       ",
        b" x    x ",
        b"  xxxx  ",
    ],
    // D
    [
        b" xxxxx  ",
        b" x    x ",
        b" x     x",
        b" x     x",
        b" x     x",
        b" x     x",
        b" x    x ",
        b" xxxxx  ",
    ],
    // E
    [
        b" xxxxxx ",
        b" x      ",
        b" x      ",
        b" xxxxx  ",
        b" x      ",
        b" x      ",
        b" x      ",
        b" xxxxxx ",
    ],
    // F
    [
        b" xxxxxx ",
        b" x      ",
        b" x      ",
        b" xxxxx  ",
        b" x      ",
        b" x      ",
        b" x      ",
        b" x      ",
    ],
    // G
    [
        b"  xxxx  ",
        b" x    x ",
        b"x       ",
        b"x       ",
        b"x  xxxx ",
        b"x     x ",
        b" x    x ",
        b"  xxxx  ",
    ],
    // H
    [
        b" x    x ",
        b" x    x ",
        b" x    x ",
        b" xxxxxx ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
    ],
    // I
    [
        b" xxxxxx ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b" xxxxxx ",
    ],
    // J
    [
        b" xxxxxx ",
        b"     x  ",
        b"     x  ",
        b"     x  ",
        b"     x  ",
        b" x   x  ",
        b" x   x  ",
        b"  xxx   ",
    ],
    // K
    [
        b" x    x ",
        b" x   x  ",
        b" x  x   ",
        b" x x    ",
        b" xxx    ",
        b" x  x   ",
        b" x   x  ",
        b" x    x ",
    ],
    // L
    [
        b" x      ",
        b" x      ",
        b" x      ",
        b" x      ",
        b" x      ",
        b" x      ",
        b" x      ",
        b" xxxxxx ",
    ],
    // M
    [
        b" x    x ",
        b" xx  xx ",
        b" x xx x ",
        b" x xx x ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
    ],
    // N
    [
        b" x    x ",
        b" xx   x ",
        b" xx   x ",
        b" x x  x ",
        b" x  x x ",
        b" x  x x ",
        b" x   xx ",
        b" x    x ",
    ],
    // O
    [
        b"  xxxx  ",
        b" x    x ",
        b"x       ",
        b"x       ",
        b"x       ",
        b"x       ",
        b" x    x ",
        b"  xxxx  ",
    ],
    // P
    [
        b" xxxxx  ",
        b" x    x ",
        b" x    x ",
        b" xxxxx  ",
        b" x      ",
        b" x      ",
        b" x      ",
        b" x      ",
    ],
    // Q
    [
        b"  xxxx  ",
        b" x    x ",
        b"x       ",
        b"x       ",
        b"x       ",
        b"x  x    ",
        b" x   x  ",
        b"  xxxx x",
    ],
    // R
    [
        b" xxxxx  ",
        b" x    x ",
        b" x    x ",
        b" xxxxx  ",
        b" x  x   ",
        b" x   x  ",
        b" x    x ",
        b" x    x ",
    ],
    // S
    [
        b"  xxxxx ",
        b" x      ",
        b" x      ",
        b"  xxxx  ",
        b"      x ",
        b"      x ",
        b"      x ",
        b" xxxxx  ",
    ],
    // T
    [
        b" xxxxxx ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
    ],
    // U
    [
        b"x    x  ",
        b"x    x  ",
        b"x    x  ",
        b"x    x  ",
        b"x    x  ",
        b"x    x  ",
        b" x  x   ",
        b"  xx    ",
    ],
    // V
    [
        b"x    x  ",
        b"x    x  ",
        b"x    x  ",
        b" x  x   ",
        b" x  x   ",
        b"  xx    ",
        b"  xx    ",
        b"   x    ",
    ],
    // W
    [
        b"x    x  ",
        b"x    x  ",
        b"x    x  ",
        b"x xx x  ",
        b"x xx x  ",
        b"xx  xx  ",
        b"xx  xx  ",
        b"x    x  ",
    ],
    // X
    [
        b"x    x  ",
        b" x  x   ",
        b"  xx    ",
        b"  xx    ",
        b"  xx    ",
        b"  xx    ",
        b" x  x   ",
        b"x    x  ",
    ],
    // Y
    [
        b"x    x  ",
        b" x  x   ",
        b"  xx    ",
        b"   x    ",
        b"   x    ",
        b"   x    ",
        b"   x    ",
        b"   x    ",
    ],
    // Z
    [
        b" xxxxxx ",
        b"      x ",
        b"     x  ",
        b"    x   ",
        b"   x    ",
        b"  x     ",
        b" x      ",
        b" xxxxxx ",
    ],
];

const LOWERCASE: [[&[u8; 8]; 8]; 26] = [
    // a
    [
        b"        ",
        b"  xxxx  ",
        b" x    x ",
        b"      x ",
        b"  xxxxx ",
        b" x    x ",
        b" x    x ",
        b"  xxxx x",
    ],
    // b
    [
        b" x      ",
        b" x      ",
        b" x xxx  ",
        b" xx   x ",
        b" x    x ",
        b" x    x ",
        b" xx   x ",
        b" xxxxx  ",
    ],
    // c
    [
        b"        ",
        b"  xxxx  ",
        b" x    x ",
        b"x       ",
        b"x       ",
        b"x       ",
        b" x    x ",
        b"  xxxx  ",
    ],
    // d
    [
        b"      x ",
        b"      x ",
        b"  xxx x ",
        b" x   xx ",
        b"x     x ",
        b"x     x ",
        b" x   xx ",
        b"  xxx x ",
    ],
    // e
    [
        b"        ",
        b"  xxxx  ",
        b" x    x ",
        b"x      x",
        b"xxxxxxx ",
        b"x       ",
        b" x    x ",
        b"  xxxx  ",
    ],
    // f
    [
        b"   xxx  ",
        b"  x   x ",
        b"  x     ",
        b" xxxxxx ",
        b"  x     ",
        b"  x     ",
        b"  x     ",
        b"  x     ",
    ],
    // g
    [
        b"        ",
        b"  xxxx  ",
        b" x    x ",
        b" x    x ",
        b"  xxxxx ",
        b"      x ",
        b" x    x ",
        b"  xxxx  ",
    ],
    // h
    [
        b" x      ",
        b" x      ",
        b" x xxx  ",
        b" xx   x ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
    ],
    // i
    [
        b"   x    ",
        b"        ",
        b"  xx    ",
        b"   x    ",
        b"   x    ",
        b"   x    ",
        b"   x    ",
        b"  xxx   ",
    ],
    // j
    [
        b"    x   ",
        b"        ",
        b"   xx   ",
        b"    x   ",
        b"    x   ",
        b"    x   ",
        b"x   x   ",
        b" xxx    ",
    ],
    // k
    [
        b" x      ",
        b" x      ",
        b" x   x  ",
        b" x  x   ",
        b" x x    ",
        b" xx x   ",
        b" x  x   ",
        b" x   x  ",
    ],
    // l
    [
        b"  xx    ",
        b"   x    ",
        b"   x    ",
        b"   x    ",
        b"   x    ",
        b"   x    ",
        b"   x    ",
        b"  xxx   ",
    ],
    // m
    [
        b"        ",
        b" xx xx  ",
        b" x x  x ",
        b" x x  x ",
        b" x x  x ",
        b" x x  x ",
        b" x x  x ",
        b" x x  x ",
    ],
    // n
    [
        b"        ",
        b" x xxx  ",
        b" xx   x ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
    ],
    // o
    [
        b"        ",
        b"  xxxx  ",
        b" x    x ",
        b"x      x",
        b"x      x",
        b"x      x",
        b" x    x ",
        b"  xxxx  ",
    ],
    // p
    [
        b"        ",
        b" xxxxx  ",
        b" x    x ",
        b" x    x ",
        b" xxxxx  ",
        b" x      ",
        b" x      ",
        b" x      ",
    ],
    // q
    [
        b"        ",
        b"  xxxx  ",
        b" x    x ",
        b"x     x ",
        b"x     x ",
        b"x    x  ",
        b"  xxxx  ",
        b"      x ",
    ],
    // r
    [
        b"        ",
        b" x xxx  ",
        b" xx   x ",
        b" x      ",
        b" x      ",
        b" x      ",
        b" x      ",
        b" x      ",
    ],
    // s
    [
        b"        ",
        b"  xxxxx ",
        b" x      ",
        b" x      ",
        b"  xxxx  ",
        b"      x ",
        b"      x ",
        b" xxxxx  ",
    ],
    // t
    [
        b"  x     ",
        b"  x     ",
        b" xxxxxx ",
        b"  x     ",
        b"  x     ",
        b"  x     ",
        b"  x   x ",
        b"   xxx  ",
    ],
    // u
    [
        b"        ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
        b" x    x ",
        b" x   xx ",
        b"  xxx x ",
    ],
    // v
    [
        b"        ",
        b"x      x",
        b"x      x",
        b" x    x ",
        b" x    x ",
        b"  x  x  ",
        b"  x  x  ",
        b"   xx   ",
    ],
    // w
    [
        b"        ",
        b"x      x",
        b"x      x",
        b"x  xx  x",
        b"x  xx  x",
        b"x  xx  x",
        b" xx  xx ",
        b" x    x ",
    ],
    // x
    [
        b"        ",
        b"x      x",
        b" x    x ",
        b"  x  x  ",
        b"   xx   ",
        b"  x  x  ",
        b" x    x ",
        b"x      x",
    ],
    // y
    [
        b"        ",
        b"x      x",
        b" x    x ",
        b" x    x ",
        b"  x  x  ",
        b"  x  x  ",
        b"   xx   ",
        b"   x    ",
    ],
    // z
    [
        b"        ",
        b" xxxxxx ",
        b"      x ",
        b"     x  ",
        b"    x   ",
        b"   x    ",
        b"  x     ",
        b" xxxxxx ",
    ],
];

const SPACE: [&[u8; 8]; 8] = [b"        "; 8];

const PUNCTUATION: [[&[u8; 8]; 8]; 18] = [
    // (
    [
        b"    x   ",
        b"   x    ",
        b"  x     ",
        b"  x     ",
        b"  x     ",
        b"  x     ",
        b"   x    ",
        b"    x   ",
    ],
    // )
    [
        b"  x     ",
        b"   x    ",
        b"    x   ",
        b"    x   ",
        b"    x   ",
        b"    x   ",
        b"   x    ",
        b"  x     ",
    ],
    // ,
    [
        b"        ",
        b"        ",
        b"        ",
        b"        ",
        b"        ",
        b"   xx   ",
        b"   xx   ",
        b"  xx    ",
    ],
    // .
    [
        b"        ",
        b"        ",
        b"        ",
        b"        ",
        b"        ",
        b"   xx   ",
        b"   xx   ",
        b"        ",
    ],
    // !
    [
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"        ",
        b"   xx   ",
        b"   xx   ",
    ],
    // ?
    [
        b"  xxxx  ",
        b" x    x ",
        b"      x ",
        b"     x  ",
        b"   xx   ",
        b"   xx   ",
        b"        ",
        b"   xx   ",
    ],
    // /
    [
        b"      x ",
        b"      x ",
        b"     x  ",
        b"    x   ",
        b"   x    ",
        b"  x     ",
        b" x      ",
        b"x       ",
    ],
    // >
    [
        b"x       ",
        b" x      ",
        b"  x     ",
        b"   x    ",
        b"  x     ",
        b" x      ",
        b"x       ",
        b"        ",
    ],
    // <
    [
        b"       x",
        b"      x ",
        b"     x  ",
        b"    x   ",
        b"     x  ",
        b"      x ",
        b"       x",
        b"        ",
    ],
    // |
    [
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
        b"   xx   ",
    ],
    // &
    [
        b"  xxx   ",
        b" x   x  ",
        b" x   x  ",
        b"  xxx   ",
        b" x x x  ",
        b"x  x x  ",
        b"x   x   ",
        b" xxx xx ",
    ],
    // *
    [
        b"        ",
        b"x  x  x ",
        b" xx xx  ",
        b"  xxx   ",
        b" xx xx  ",
        b"x  x  x ",
        b"        ",
        b"        ",
    ],
    // =
    [
        b"        ",
        b"        ",
        b" xxxxxx ",
        b"        ",
        b" xxxxxx ",
        b"        ",
        b"        ",
        b"        ",
    ],
    // -
    [
        b"        ",
        b"        ",
        b"        ",
        b" xxxxxx ",
        b"        ",
        b"        ",
        b"        ",
        b"        ",
    ],
    // :
    [
        b"        ",
        b"   xx   ",
        b"   xx   ",
        b"        ",
        b"        ",
        b"   xx   ",
        b"   xx   ",
        b"        ",
    ],
    // #
    [
        b"        ",
        b"  x  x  ",
        b" xxxxxx ",
        b"  x  x  ",
        b" xxxxxx ",
        b"  x  x  ",
        b"        ",
        b"        ",
    ],
    // ♯
    [
        b"   x  x ",
        b"  x  x  ",
        b"  xxxxx ",
        b"   xxxxx",
        b"  x  x  ",
        b" x  x   ",
        b" x  x   ",
        b"        ",
    ],
    // ♭
    [
        b"  xx    ",
        b"  x     ",
        b"  x     ",
        b"  x xx  ",
        b"  xx  x ",
        b"  x   x ",
        b"   xxx  ",
        b"        ",
    ],
];

pub struct Font {}

impl Font {
    pub fn write_digit(s: &mut Screen, y: usize, x: usize, num: usize, scale: usize) {
        Self::write_glyph(s, y, x, &DIGITS[num], scale);
    }

    pub fn write_char(s: &mut Screen, y: usize, x: usize, character: char, scale: usize) -> bool {
        let glyph = match character {
            '0'..='9' => &DIGITS[character as usize - '0' as usize],
            'A'..='Z' => &LETTERS[character as usize - 'A' as usize],
            'a'..='z' => &LOWERCASE[character as usize - 'a' as usize],
            ' ' => &SPACE,
            '(' => &PUNCTUATION[0],
            ')' => &PUNCTUATION[1],
            ',' => &PUNCTUATION[2],
            '.' => &PUNCTUATION[3],
            '!' => &PUNCTUATION[4],
            '?' => &PUNCTUATION[5],
            '/' => &PUNCTUATION[6],
            '>' => &PUNCTUATION[7],
            '<' => &PUNCTUATION[8],
            '|' => &PUNCTUATION[9],
            '&' => &PUNCTUATION[10],
            '*' => &PUNCTUATION[11],
            '=' => &PUNCTUATION[12],
            '-' => &PUNCTUATION[13],
            ':' => &PUNCTUATION[14],
            '#' => &PUNCTUATION[15],
            '♯' => &PUNCTUATION[16],
            '♭' => &PUNCTUATION[17],
            _ => return false,
        };

        Self::write_glyph(s, y, x, glyph, scale);
        true
    }

    fn write_glyph(s: &mut Screen, y: usize, x: usize, glyph: &[&[u8; 8]; 8], scale: usize) {
        for i in 0..(8 * scale) {
            for j in 0..(8 * scale) {
                let bit = glyph[i / scale][j / scale] != b' ';
                s.set(i + y, j + x, bit);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Font;
    use crate::screen::Screen;

    #[test]
    fn writes_all_supported_characters() {
        let mut screen = Screen::new();
        for character in
            "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz (),.!?/><|&*=-:#♯♭"
                .chars()
        {
            assert!(Font::write_char(&mut screen, 0, 0, character, 1));
        }
    }

    #[test]
    fn unsupported_characters_are_not_written() {
        let mut screen = Screen::new();
        assert!(!Font::write_char(&mut screen, 0, 0, '{', 1));
    }

    #[test]
    fn writing_a_space_clears_its_glyph_area() {
        let mut screen = Screen::new();
        assert!(Font::write_char(&mut screen, 0, 0, 'A', 1));
        assert!(screen.get(0, 3));

        assert!(Font::write_char(&mut screen, 0, 0, ' ', 1));
        assert!(!screen.get(0, 3));
    }
}
