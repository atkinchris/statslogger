use crate::stats::Stats;
use clap::arg_enum;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

mod stats;

arg_enum! {
  #[derive(Debug)]
  enum Format {
      Plain,
      JSON,
  }
}

impl Default for Format {
  fn default() -> Self {
    Format::JSON
  }
}

#[derive(StructOpt)]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opts {
  /// Output format
  #[structopt(short, long, possible_values = &Format::variants(), case_insensitive = true, default_value)]
  format: Format,

  /// Set frequency time in seconds
  #[structopt(short, long, default_value = "5")]
  time: u64,

  /// Output results to file, in format specified
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
    thread::sleep(Duration::from_secs(opt.time));
    stats.tick();

    let output = match &opt.format {
      Format::Plain => stats.to_string(),
      Format::JSON => stats.to_json(),
    };

    if let Some(output_file) = &mut output_file {
      writeln!(output_file, "{}", output)
        .unwrap_or_else(|err| println!("Could not write to file: {}", err));
    }

    println!("{}", output);
  }
}
