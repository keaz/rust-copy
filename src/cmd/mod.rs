use clap::Parser;


/// CLI application copy file with resume capability
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
    /// File that needs to be copy
    #[arg(short, long,default_value_t = String::from("/Users/kasunranasinghe/Development/RUST/test/123.mkv"))]
    pub source: String,

    /// Destination folder
    #[arg(short, long, default_value_t = String::from("/Users/kasunranasinghe/Development/RUST/test_2"))]
    pub destination: String,
}
