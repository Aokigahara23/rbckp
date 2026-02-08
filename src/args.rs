use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Allow invalid UTF-8 paths
    #[arg(short = 'F', value_name = "file", value_hint = clap::ValueHint::DirPath)]
    pub target_file: std::path::PathBuf,
}
