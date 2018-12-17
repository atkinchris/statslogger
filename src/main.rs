extern crate chrono;
extern crate sysinfo;

use chrono::Local;
use std::thread;
use std::time::Duration;
use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};

#[derive(Debug)]
struct Stats {
  cpu_temp: f32,
  cpu_usage: f32,
  timestamp: String,
}

impl Stats {
  fn to_string(&self) -> String {
    format!("{}, {}, {}", self.timestamp, self.cpu_usage, self.cpu_temp)
  }
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

fn tick(sys: &mut System) {
  sys.refresh_system();

  let stats = Stats {
    cpu_temp: get_cpu_temperature(&sys),
    cpu_usage: get_cpu_percentage(&sys),
    timestamp: get_timestamp(),
  };

  println!("{}", stats.to_string());
}

fn main() {
  let mut sys = System::new();
  sys.refresh_all();
  thread::sleep(Duration::from_secs(1));

  loop {
    tick(&mut sys);
    thread::sleep(Duration::from_secs(5));
  }
}
