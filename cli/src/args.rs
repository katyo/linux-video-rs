use linux_video::types;

#[derive(clap::Parser)]
#[command(
    name = "linux-video",
    version,
    about,
    propagate_version = true,
    // Command::trailing_var_ar is required to use ValueHint::CommandWithArguments
    trailing_var_arg = true,
)]
pub struct Args {
    /// Video commands
    #[clap(subcommand)]
    pub cmd: Cmd,
}

#[derive(clap::Parser)]
pub enum Cmd {
    /// List video devices
    List,

    /// Get info about video device
    Info {
        /// Show capabilities
        #[arg(short = 'a', long)]
        capabilities: bool,

        /// Show controls
        #[arg(short = 'o', long)]
        controls: bool,

        /// Show supported formats
        #[arg(short, long)]
        formats: bool,

        /// Show supported frame sizes
        #[arg(short = 's', long, value_enum)]
        sizes: bool,

        /// Show supported frame intervals
        #[arg(short = 'i', long, value_enum)]
        intervals: bool,

        /// Show all available info (shortcut for -aofsi)
        #[arg(short = 'l', long)]
        all: bool,

        /// Only controls of specified classes
        #[arg(short, long, value_enum)]
        class: Vec<types::CtrlClass>,

        /// Only formats for specified buffer types
        #[arg(short, long, value_enum)]
        r#type: Vec<types::BufferType>,

        /// Video device paths or names (ex. video0)
        #[arg(value_parser)]
        devices: Vec<String>,
    },
}
