use crate::hash;
use chrono::Local;
use serde::Serialize;
use sysinfo::{ComponentExt, ProcessExt, ProcessorExt, RefreshKind, System, SystemExt};
use whoami::{hostname, os, platform, username};

#[derive(Default, Serialize, Clone)]
pub struct Process {
  pub name: String,
  pub cpu_usage: f32,
}

#[derive(Default, Serialize)]
pub struct Stats {
  #[serde(skip_serializing)]
  sys: System,
  pub cpu_temp: Option<f32>,
  pub cpu_usage: Option<f32>,
  pub mem_usage: f32,
  pub timestamp: String,
  pub hostname: String,
  pub username: String,
  pub os: String,
  pub platform: String,
  pub top_processes: Vec<Process>,
}

impl Stats {
  pub fn to_string(&self) -> String {
    format!(
      "{}, {}, {}, {}, {}, {:.0}%, {}, {:.0}%, {}",
      self.hostname,
      self.username,
      self.timestamp,
      self.platform,
      self.os,
      match self.cpu_usage {
        Some(cpu_usage) => format!("{:.0}%", cpu_usage),
        None => "N/A".to_owned(),
      },
      match self.cpu_temp {
        Some(cpu_temp) => format!("{:.0}C", cpu_temp),
        None => "N/A".to_owned(),
      },
      self.mem_usage,
      &self
        .top_processes
        .iter()
        .map(|process| format!("{} ({:.1}%)", &process.name, &process.cpu_usage))
        .collect::<Vec<String>>()
        .join(","),
    )
  }

  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

  pub fn tick(&mut self, max_processes: usize) {
    self.sys.refresh_all();

    self.cpu_temp = get_cpu_temperature(&self.sys);
    self.cpu_usage = get_cpu_percentage(&self.sys);
    self.mem_usage = get_mem_percentage(&self.sys);
    self.top_processes = get_top_processes(&self.sys, max_processes);
    self.timestamp = get_timestamp();
  }

  pub fn create(hash_pii: bool) -> Stats {
    let mut sys = System::new_with_specifics(
      RefreshKind::new()
        .with_cpu()
        .with_components()
        .with_components_list()
        .with_memory()
        .with_processes(),
    );
    &sys.refresh_all();

    let mut username = username();

    if hash_pii {
      username = hash::hash_string(username);
    }

    Stats {
      sys,
      username,
      hostname: hostname(),
      platform: platform().to_string(),
      os: os(),
      ..Default::default()
    }
  }
}

fn get_mem_percentage(sys: &System) -> f32 {
  (sys.get_used_memory() as f32 / sys.get_total_memory() as f32) * 100.0
}

fn get_cpu_temperature(sys: &System) -> Option<f32> {
  sys
    .get_components()
    .into_iter()
    .find(|&component| component.get_label() == "CPU")
    .and_then(|component| Some(component.get_temperature()))
}

fn get_cpu_percentage(sys: &System) -> Option<f32> {
  let do_steps = || -> Option<f32> {
    let cpu_usage = sys.get_global_processor_info().get_cpu_usage();
    Some(cpu_usage)
  };

  do_steps()
}

fn get_top_processes(sys: &System, max_processes: usize) -> Vec<Process> {
  if max_processes <= 0 {
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

  if processes.len() > max_processes {
    processes = processes[..max_processes].to_vec();
  }

  processes
}

fn get_timestamp() -> String {
  Local::now().to_rfc3339()
}
