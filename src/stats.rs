use chrono::Local;
use serde::Serialize;
use sysinfo::{ComponentExt, ProcessorExt, RefreshKind, System, SystemExt};
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
}

impl Stats {
  pub fn to_string(&self) -> String {
    format!(
      "{}, {}, {}, {:.0}%, {:.0}C, {:.0}%",
      self.hostname, self.username, self.timestamp, self.cpu_usage, self.cpu_temp, self.mem_usage
    )
  }

  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

  pub fn tick(&mut self) {
    self.sys.refresh_system();

    self.cpu_temp = get_cpu_temperature(&self.sys);
    self.cpu_usage = get_cpu_percentage(&self.sys);
    self.mem_usage = get_mem_percentage(&self.sys);
    self.timestamp = get_timestamp();
  }

  pub fn create() -> Stats {
    let mut sys = System::new_with_specifics(
      RefreshKind::new()
        .with_cpu()
        .with_components()
        .with_components_list()
        .with_memory(),
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
  sys.get_global_processor_info().get_cpu_usage() * 100.0
}

fn get_timestamp() -> String {
  Local::now().to_rfc3339()
}
