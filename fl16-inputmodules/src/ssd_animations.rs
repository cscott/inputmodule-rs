use crate::patterns::*;
use crate::ssd::Grid;
use crate::ssd::*;

// TODO
// - [ ] Is there a cancellable Iterator? I think Java/Kotlin has one
// - [ ] Each one has a number of frames
// - [ ] Each one might have a different frame-rate

#[allow(clippy::large_enum_variant)]
pub enum Animation {
    Percentage(StartupPercentageIterator),
    Breathing(BreathingIterator),
    Spin(SpinIterator),
    Wave(WaveIterator),
}
impl Iterator for Animation {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Animation::Percentage(x) => x.next(),
            Animation::Breathing(x) => x.next(),
            Animation::Spin(x) => x.next(),
            Animation::Wave(x) => x.next(),
        }
    }
}

pub struct StartupPercentageIterator {
    frames: usize,
    current_frame: usize,
}

impl Default for StartupPercentageIterator {
    fn default() -> Self {
        Self {
            frames: WIDTH * HEIGHT,
            current_frame: 0,
        }
    }
}

impl Iterator for StartupPercentageIterator {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_frame < self.frames {
            self.current_frame += 1;
            let mut grid = Grid::default();
            let mut i = 0;
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    grid.0[x][y] = if i < self.current_frame { 0xFF } else { 0x00 };
                    i = i + 1;
                }
            }
            Some(grid)
        } else {
            None
        }
    }
}

pub struct BreathingIterator {
    frames_remaining: usize,
    current_brightness: u8,
}

impl BreathingIterator {
    pub fn new(frames: usize) -> Self {
        Self {
            frames_remaining: frames,
            current_brightness: 0,
        }
    }
}
impl Default for BreathingIterator {
    fn default() -> Self {
        Self::new(64)
    }
}

impl Iterator for BreathingIterator {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frames_remaining > 0 {
            let mut grid = Grid::default();
            let breath_step = 4;
            // TODO: Make it cycle up and down
            self.current_brightness = (self.current_brightness + breath_step) % 255;
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    grid.0[x][y] = self.current_brightness;
                }
            }
            self.frames_remaining -= 1;
            Some(grid)
        } else {
            None
        }
    }
}

pub struct SpinIterator {
    frames: usize,
    current_frame: usize,
}

impl Default for SpinIterator {
    fn default() -> Self {
        Self {
            frames: 128,
            current_frame: 0,
        }
    }
}

impl Iterator for SpinIterator {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_frame < self.frames {
            self.current_frame += 1;
            let progress = 6 * self.current_frame / self.frames;
            Some(spin(self.current_frame, progress))
        } else {
            None
        }
    }
}

pub struct WaveIterator {
    frames: usize,
    current_frame: usize,
}

impl WaveIterator {
    pub fn new(frames: usize) -> Self {
        Self {
            frames,
            current_frame: 0,
        }
    }
}

impl Default for WaveIterator {
    fn default() -> Self {
        Self::new(128)
    }
}

impl Iterator for WaveIterator {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_frame < self.frames {
            self.current_frame += 1;
            let mut grid = Grid::default();
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let index = (x + WIDTH * y + self.current_frame) % (WIDTH * HEIGHT);
                    let brightness = index * 3;
                    grid.0[x][y] = if brightness > 255 { 0 } else { brightness } as u8;
                }
            }
            Some(grid)
        } else {
            None
        }
    }
}
