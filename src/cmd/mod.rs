use clap::Parser;


/// CLI application copy file with resume capability
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
    /// File that needs to be copy
    #[arg(short, long)]
    pub source: String,

    /// Destination folder
    #[arg(short, long)]
    pub destination: String,
}
