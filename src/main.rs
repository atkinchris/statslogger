use crate::stats::Stats;

use clap::arg_enum;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

mod filelogging;
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
      filelogging::write_to_file(
        output_path,
        &format!("statslogger_{}", &stats.hostname),
        &output,
      )
      .unwrap_or_else(|err| println!("Could not write to file: {}", err));
    }

    println!("{}", output);
  }
}
