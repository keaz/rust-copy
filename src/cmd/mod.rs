use clap::Parser;

use crate::{DEFAULT_BUF_SIZE, DEFAULT_THREAD_COUNT,DEFAULT_READ_THREAD_COUNT};

/// A CLI tool that replaces the cp
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CmdArgs {
    /// File that needs to be copy
    #[arg(short, long)]
    pub source: String,

    /// Destination folder
    #[arg(short, long)]
    pub destination: String,

    /// Number of threads to use read the filed
    #[arg(short, long, default_value_t=DEFAULT_READ_THREAD_COUNT)]
    pub read_thread: i8,

    /// Number of threads to use copy files
    #[arg(short,long,default_value_t=DEFAULT_THREAD_COUNT)]
    pub threads: i8,

    /// Size of the buffer
    #[arg(short,long, default_value_t=DEFAULT_BUF_SIZE)]
    pub buffer_size: u32,
}
