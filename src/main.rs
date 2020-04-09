use crate::stats::Stats;
use chrono::Local;
use clap::arg_enum;
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

  /// Output logs to a folder, in files grouped by current date and hour.
  #[structopt(short, long)]
  output: Option<String>,

  /// Number of processes to log
  #[structopt(short, long, default_value = "10")]
  processes: usize,
}

fn open_output_file(file_path: &String, filename_prefix: &String) -> File {
  let date = Local::now().format("%Y%m%d_%H").to_string();
  let filename = &format!("statslogger_{}_{}", filename_prefix, date);
  let path = Path::new(file_path).join(filename);
  let compound_path = path.to_str().expect(&format!(
    "Could not join file paths: {} and {}",
    &file_path, &filename
  ));

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

    if let Some(output_path) = &opt.output {
      let mut output_file = open_output_file(output_path, &stats.hostname);
      writeln!(output_file, "{}", output)
        .unwrap_or_else(|err| println!("Could not write to file: {}", err));
    }

    println!("{}", output);
  }
}
