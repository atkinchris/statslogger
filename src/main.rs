use crate::stats::Stats;

use clap::arg_enum;
use dirs;
use dotenv;
use log::{debug, error};
use std::env;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

mod filelogging;
mod httplogging;
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

  /// Output logs to a folder, in files grouped by current date and hour
  #[structopt(short, long)]
  output: Option<String>,

  /// Post logs to a URL, in JSON format
  #[structopt(short, long, env = "STATSLOGGER_URL", hide_env_values = true)]
  url: Option<String>,

  /// Maximum number of processes to log
  #[structopt(short, long, default_value = "10")]
  processes: usize,

  /// Disable hashing of username
  #[structopt(long)]
  no_hashing: bool,

  /// Show debug messages
  #[structopt(long)]
  debug: bool,
}

fn main() {
  let config_path = dirs::home_dir()
    .and_then(|a| Some(a.join(".statslogger")))
    .unwrap();
  let config_read_result = dotenv::from_path(config_path);

  let opt = Opts::from_args();
  let mut stats = Stats::create(!opt.no_hashing);

  stderrlog::new()
    .module(module_path!())
    .verbosity(if opt.debug { 4 } else { 1 })
    .init()
    .unwrap();

  config_read_result.unwrap_or_else(|err| debug!("Could not read from to file. {}", err));

  loop {
    thread::sleep(Duration::from_secs(opt.time));
    stats.tick(opt.processes);

    let output = match &opt.format {
      Format::Plain => stats.to_string(),
      Format::JSON => stats.to_json(),
    };

    if let Some(output_path) = &opt.output {
      debug!("Writing to \"{}\"", output_path);
      filelogging::write_to_file(
        output_path,
        &format!("statslogger_{}", &stats.hostname),
        &output,
      )
      .unwrap_or_else(|err| error!("Could not write to file. {}", err));
    }

    if let Some(url) = &opt.url {
      debug!("Posting to \"{}\"", url);
      httplogging::post_to_url(url, stats.to_json())
        .unwrap_or_else(|err| error!("Could not post to url: {}", err));
    }

    println!("{}", output);
  }
}
