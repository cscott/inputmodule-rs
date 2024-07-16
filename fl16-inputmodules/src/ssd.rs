/// Seven Segment Display Input Module
use crate::control::PwmFreqArg;
use crate::ssd_animations::Animation;
use smart_leds::RGB8;

pub const WIDTH: usize = 8; // 7 segments plus a decimal point
pub const HEIGHT: usize = 9;
pub const LEDS: usize = WIDTH * HEIGHT;

#[derive(Clone)]
pub struct Grid(pub [[u8; HEIGHT]; WIDTH]);
impl Default for Grid {
    fn default() -> Self {
        Grid([[0; HEIGHT]; WIDTH])
    }
}

impl Grid {
    pub fn rotate(&mut self, rotations: usize) {
        for x in 0..WIDTH {
            self.0[x].rotate_right(rotations);
        }
    }
}

pub struct SevenSegmentState {
    /// Currently displayed grid
    pub grid: Grid,
    /// Temporary buffer for building a new grid
    pub col_buffer: Grid,
    /// Whether the grid is currently being animated
    pub animate: bool,
    /// LED brightness out of 255
    pub brightness: u8,
    /// Neopixel colors
    pub color: [RGB8; 5],
    /// Current sleep state
    pub sleeping: SleepState,
    pub animation_period: u64,
    /// Current LED PWM frequency
    pub pwm_freq: PwmFreqArg,
    /// Whether debug mode is active
    ///
    /// In debug mode:
    /// - Startup is instant, no animation
    /// - Sleep/wake transition is instant, no animation/fading
    /// - No automatic sleeping
    pub debug_mode: bool,
    pub upcoming_frames: Option<Animation>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
/// Whether asleep or not, if asleep contains data to restore previous LED grid
pub enum SleepState {
    Awake,
    Sleeping((Grid, u8)),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SleepReason {
    Command,
    SleepPin,
    Timeout,
    UsbSuspend,
}

/// Some letters!
pub fn convert_font(c: char) -> [bool; 7] {
    match c {
        '0' => [true, true, true, true, true, true, false],
        '1' => [false, true, true, false, false, false, false],
        '2' => [true, true, false, true, true, false, true],
        '3' => [true, true, true, true, false, false, true],
        '4' => [false, true, true, false, false, true, true],
        '5' => [true, false, true, true, false, true, true],
        '6' => [true, false, true, true, true, true, true],
        '7' => [true, true, true, false, false, false, false],
        '8' => [true, true, true, true, true, true, true],
        '9' => [true, true, true, false, false, true, true],
        'A' => [true, true, true, false, true, true, true],
        'B' => [false, false, true, true, true, true, true], // b
        'C' => [true, false, false, true, true, true, false],
        'c' => [false, false, false, true, true, false, true],
        'D' => [false, true, true, true, true, false, true], // d
        'E' => [true, false, false, true, true, true, true],
        'F' => [true, false, false, false, true, true, true],
        'G' => [true, false, true, true, true, true, false],
        'H' => [false, true, true, false, true, true, true],
        'h' => [false, false, true, false, true, true, true],
        'I' => [false, false, true, false, false, false, false], // i
        'J' => [false, true, true, true, true, false, false],
        'K' => [true, false, true, false, true, true, true], // questionable
        'L' => [false, false, false, true, true, true, false],
        'M' => [true, true, true, false, true, true, false], // questionable
        'N' => [false, false, true, false, true, false, true], // n
        'O' => [true, true, true, true, true, true, false],  // same as 0
        'o' => [false, false, true, true, true, false, true],
        'P' => [true, true, false, false, true, true, true],
        'Q' => [true, true, true, false, false, true, true], // q
        'R' => [false, false, false, false, true, false, true], // r
        'S' => [true, false, true, true, false, true, true], // same as 5
        'T' => [false, false, false, true, true, true, true], // t
        'U' => [false, false, true, true, true, false, false], // u
        'V' => [false, true, true, true, true, true, false], // questionable
        'W' => [false, true, true, true, true, true, true],  // questionable
        'X' => [false, true, true, false, true, true, true], // same as H
        'Y' => [false, true, true, true, false, true, true],
        'Z' => [true, true, false, true, true, false, false],
        ' ' => [false, false, false, false, false, false, false],
        '-' => [false, false, false, false, false, false, true],
        '_' => [false, false, false, true, false, false, false],
        '=' => [false, false, false, true, false, false, true],
        '/' => [false, true, false, false, true, false, true],
        '\\' => [false, false, true, false, false, true, true],
        '?' => [true, true, false, false, true, false, true],
        '!' => [false, true, false, false, false, false, false],
        _ => {
            if c.is_ascii_lowercase() {
                convert_font(c.to_ascii_uppercase())
            } else {
                convert_font(' ')
            }
        }
    }
}

pub fn display_base36(pos: usize, grid: &mut Grid, digit: u8) {
    display_char(pos, grid, char::from_digit(digit as u32, 36).unwrap_or(' '));
}

pub fn display_char(pos: usize, grid: &mut Grid, c: char) {
    let lookup = convert_font(c);
    for seg in 0..7 {
        let val = if lookup[seg as usize] { 0xFF } else { 0 };
        grid.0[seg][pos] = val;
    }
    // clear point by default
    grid.0[7][pos] = 0x00;
}

pub fn display_point(pos: usize, grid: &mut Grid, point: bool) {
    grid.0[7][pos] = if point { 0xFF } else { 0x00 };
}

pub fn display_string(pos: usize, grid: &mut Grid, s: &str) {
    let mut p = pos;
    let mut last_was_dot = false;
    for c in s.chars() {
        if c == '.' || c == ',' || c == '!' {
            if p == pos || last_was_dot || c == '!' {
                if p >= HEIGHT {
                    break;
                }
                display_char(p, grid, if c == '!' { c } else { ' ' });
                display_point(p, grid, true);
                p = p + 1;
            } else {
                display_point(p - 1, grid, true);
            }
            last_was_dot = true;
        } else {
            if p >= HEIGHT {
                break;
            }
            display_char(p, grid, c);
            p = p + 1;
            last_was_dot = false;
        }
    }
}
