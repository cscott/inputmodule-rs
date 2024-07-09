use clap::Parser;

#[derive(Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
#[repr(u8)]
pub enum Pattern {
    Percentage = 0,
    Gradient = 1,
    DoubleGradient = 2,
    LotusSideways = 3,
    Zigzag = 4,
    AllOn = 5,
    Panic = 6,
    LotusTopDown = 7,
    //AllBrightnesses
}

#[derive(Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum Color {
    White,
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Purple,
}

/// Seven Segment Display
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
pub struct SevenSegmentSubcommand {
    /// Set LED max brightness percentage or get, if no value provided
    #[arg(long)]
    pub brightness: Option<Option<u8>>,

    /// Set sleep status or get, if no value provided
    #[arg(long)]
    pub sleeping: Option<Option<bool>>,

    /// Jump to the bootloader
    #[arg(long)]
    pub bootloader: bool,

    /// Display a percentage (0-100)
    #[arg(long)]
    pub percentage: Option<u8>,

    /// Start/stop animation
    #[arg(long)]
    pub animate: Option<Option<bool>>,

    /// Display a pattern
    #[arg(long)]
    #[clap(value_enum)]
    pub pattern: Option<Pattern>,

    /// Show every brightness, one per pixel
    #[arg(long)]
    pub all_brightnesses: bool,

    /// Blink the current pattern once a second
    #[arg(long)]
    pub blinking: bool,

    /// Breathing brightness of the current pattern
    #[arg(long)]
    pub breathing: bool,

    /// Clock
    #[arg(long)]
    pub clock: bool,

    /// Display a string (max 9-18 chars)
    #[arg(long)]
    pub string: Option<String>,

    /// Set the legend backlight color
    // TODO: Allow getting current state
    #[arg(long)]
    #[clap(value_enum)]
    pub set_color: Option<Color>,

    /// Set/get animation FPS
    #[arg(long)]
    pub animation_fps: Option<Option<u16>>,

    /// Set/get PWM Frequency in Hz
    #[arg(long)]
    #[clap(value_enum)]
    pub pwm_freq: Option<Option<u16>>,

    /// Set debug mode or get current mode, if no value provided
    #[arg(long)]
    pub debug_mode: Option<Option<bool>>,

    /// Crash the firmware (TESTING ONLY!)
    #[arg(long)]
    pub panic: bool,

    /// Get the device version
    #[arg(short, long)]
    pub version: bool,
}
