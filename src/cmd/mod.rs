use clap::Parser;

use crate::{DEFAULT_BUF_SIZE, DEFAULT_THREAD_COUNT};


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

    /// number of threads to use
    #[arg(short,long,default_value_t=DEFAULT_THREAD_COUNT)]
    pub threads: i8,

    // Buffer size,
    #[arg(short,long, default_value_t=DEFAULT_BUF_SIZE)]
    pub buffer_size: u32
}
