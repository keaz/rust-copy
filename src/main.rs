use std::{
    env,
    fmt::Write,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use clap::Parser;
use copy_file::{
    cmd::CmdArgs,
    io::{FileReader, FileWriter},
    Data,
};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    let cmds = CmdArgs::parse_from(env::args_os());

    let mut file_reader = FileReader::new(cmds.source).await;
    let size = file_reader.size().await;
    let mut file_writer = FileWriter::new(cmds.destination, file_reader.name(), size)
        .await
        .unwrap();
    let buf_size = 10240;
    let mut buf = vec![0; buf_size];
    let mut offset = 0;

    let bar = ProgressBar::new(size);
    bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let (sender, mut receiver) = channel(1000);
    let sender = Arc::new(sender);

    tokio::spawn( async move {
        while file_reader.read_random(offset, &mut buf).await.unwrap() {
            let cloned_sender = sender.clone();
            let data = Data { data: buf, offset };
            cloned_sender.send(data).await.unwrap();
            
            offset = offset + buf_size as u64;
            buf = vec![0; buf_size];
        }
    });

    println!("Red all the data");

    while let Some(data) = receiver.recv().await {
        file_writer
            .write_random(data.offset, &data.data)
            .await
            .unwrap();
        bar.inc(data.data.len() as u64);
    }
}
