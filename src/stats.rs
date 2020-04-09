use chrono::Local;
use serde::Serialize;
use sysinfo::{ComponentExt, ProcessExt, ProcessorExt, RefreshKind, System, SystemExt};
use whoami::{hostname, username};

#[derive(Default, Serialize)]
pub struct Stats {
  #[serde(skip_serializing)]
  sys: System,
  cpu_temp: f32,
  cpu_usage: f32,
  mem_usage: f32,
  timestamp: String,
  hostname: String,
  username: String,
  top_processes: Vec<Process>,
}

#[derive(Default, Serialize, Clone)]
struct Process {
  name: String,
  cpu_usage: f32,
}

impl Stats {
  pub fn to_string(&self) -> String {
    format!(
      "{}, {}, {}, {:.0}%, {:.0}C, {:.0}%, {}",
      self.hostname,
      self.username,
      self.timestamp,
      self.cpu_usage,
      self.cpu_temp,
      self.mem_usage,
      self
        .top_processes
        .clone()
        .into_iter()
        .map(|process| format!("{} ({:.1}%)", process.name, process.cpu_usage))
        .collect::<Vec<String>>()
        .join(",")
    )
  }

  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

  pub fn tick(&mut self, number_of_processes: usize) {
    self.sys.refresh_all();

    self.cpu_temp = get_cpu_temperature(&self.sys);
    self.cpu_usage = get_cpu_percentage(&self.sys);
    self.mem_usage = get_mem_percentage(&self.sys);
    self.top_processes = get_top_processes(&self.sys, number_of_processes);
    self.timestamp = get_timestamp();
  }

  pub fn create() -> Stats {
    let mut sys = System::new_with_specifics(
      RefreshKind::new()
        .with_cpu()
        .with_components()
        .with_components_list()
        .with_memory()
        .with_processes(),
    );
    &sys.refresh_all();

    Stats {
      sys,
      hostname: hostname(),
      username: username(),
      ..Default::default()
    }
  }
}

fn get_mem_percentage(sys: &System) -> f32 {
  (sys.get_used_memory() as f32 / sys.get_total_memory() as f32) * 100.0
}

fn get_cpu_temperature(sys: &System) -> f32 {
  sys
    .get_components()
    .into_iter()
    .find(|&component| component.get_label() == "CPU")
    .expect("Unable to find CPU component")
    .get_temperature()
}

fn get_cpu_percentage(sys: &System) -> f32 {
  sys.get_global_processor_info().get_cpu_usage()
}

fn get_top_processes(sys: &System, number_of_processes: usize) -> Vec<Process> {
  if number_of_processes == 0 {
    return Vec::new();
  }

  let mut processes: Vec<Process> = sys
    .get_processes()
    .values()
    .map(|process| Process {
      name: process.name().to_owned(),
      cpu_usage: process.cpu_usage(),
    })
    .collect();

  processes.sort_by(|a, b| {
    b.cpu_usage
      .partial_cmp(&a.cpu_usage)
      .expect("Error sorting processes")
  });

  processes[..number_of_processes].to_vec()
}

fn get_timestamp() -> String {
  Local::now().to_rfc3339()
}