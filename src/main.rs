use crate::stats::Stats;
use chrono::Local;
use clap::arg_enum;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
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

  /// Output results to file, in format specified, appended with current date and hour.
  ///
  /// For example: --output log.ndjson => log_2020040914.ndjson
  #[structopt(short, long)]
  output: Option<String>,

  /// Number of processes to log
  #[structopt(short, long, default_value = "10")]
  processes: usize,
}

fn open_output_file(file_path: &String) -> File {
  let extension = Path::new(file_path)
    .extension()
    .and_then(OsStr::to_str)
    .unwrap();
  let date = Local::now().format("%Y%m%d%H").to_string();
  let compound_path = file_path.replace(
    &format!(".{}", extension),
    &format!("_{}.{}", date, extension),
  );

  OpenOptions::new()
    .create(true)
    .append(true)
    .open(&compound_path)
    .expect(&format!("Could not open file \"{}\"", &compound_path))
}

fn main() {
  let opt = Opts::from_args();
  let mut stats = Stats::create();

  loop {
    thread::sleep(Duration::from_secs(opt.time));
    stats.tick(opt.processes);

    let output = match &opt.format {
      Format::Plain => stats.to_string(),
      Format::JSON => stats.to_json(),
    };

    if let Some(output_file_path) = &opt.output {
      let mut output_file = open_output_file(output_file_path);
      writeln!(output_file, "{}", output)
        .unwrap_or_else(|err| println!("Could not write to file: {}", err));
    }

    println!("{}", output);
  }
}
