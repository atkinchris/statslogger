use crate::stats::Stats;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

mod stats;

#[derive(StructOpt)]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opts {
  /// Output as JSON
  #[structopt(short, long)]
  json: bool,

  /// Set frequency in seconds
  #[structopt(short, long, default_value = "5")]
  frequency: u64,

  /// Output results to file: {hostname}, {timestamp}, {CPU}%, {temp}C, {MEM}%
  #[structopt(short, long)]
  output: Option<String>,
}

fn open_output_file(file_path: &Option<String>) -> Option<File> {
  match file_path {
    Some(file_path) => {
      let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();

      Some(file)
    }
    None => None,
  }
}

fn main() {
  let opt = Opts::from_args();
  let mut output_file = open_output_file(&opt.output);
  let mut stats = Stats::create();

  loop {
    thread::sleep(Duration::from_secs(opt.frequency));
    stats.tick();

    if let Some(output_file) = &mut output_file {
      writeln!(output_file, "{}", stats.to_string())
        .unwrap_or_else(|err| println!("Could not write to file: {}", err));
    }

    match &opt {
      Opts { json: true, .. } => {
        println!("{}", stats.to_json());
      }
      Opts { json: false, .. } => {
        println!("{}", stats.to_string());
      }
    }
  }
}
