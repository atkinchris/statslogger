use chrono::Local;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;
use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};

#[derive(StructOpt)]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opts {
  /// Output as JSON
  #[structopt(short, long)]
  json: bool,

  /// Set frequency in seconds
  #[structopt(short, long, default_value = "5")]
  frequency: u64,

  /// Output results to file: {timestamp}, {CPU}%, {temp}C, {MEM}%
  #[structopt(short, long)]
  output: Option<String>,
}

#[derive(Debug, Serialize)]
struct Stats {
  cpu_temp: f32,
  cpu_usage: f32,
  mem_usage: f32,
  timestamp: String,
}

impl Stats {
  fn to_string(&self) -> String {
    format!(
      "{}, {:.0}%, {:.0}C, {:.0}%",
      self.timestamp, self.cpu_usage, self.cpu_temp, self.mem_usage
    )
  }

  fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }
}

fn get_mem_percentage(sys: &System) -> f32 {
  (sys.get_used_memory() as f32 / sys.get_total_memory() as f32) * 100.0
}

fn get_cpu_temperature(sys: &System) -> f32 {
  sys
    .get_components_list()
    .into_iter()
    .find(|&component| component.get_label() == "CPU")
    .unwrap()
    .get_temperature()
}

fn get_cpu_percentage(sys: &System) -> f32 {
  sys.get_processor_list()[0].get_cpu_usage() * 100.0
}

fn get_timestamp() -> String {
  Local::now().to_rfc3339()
}

fn tick(sys: &mut System) -> Stats {
  sys.refresh_system();

  Stats {
    cpu_temp: get_cpu_temperature(&sys),
    cpu_usage: get_cpu_percentage(&sys),
    mem_usage: get_mem_percentage(&sys),
    timestamp: get_timestamp(),
  }
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

  let mut sys = System::new();
  sys.refresh_all();

  thread::sleep(Duration::from_secs(1));

  loop {
    let stats = tick(&mut sys);

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

    thread::sleep(Duration::from_secs(opt.frequency));
  }
}
