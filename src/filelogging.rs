use chrono::Local;
use std::fs::OpenOptions;
use std::io::{prelude::*, Error};
use std::path::Path;

pub fn write_to_file(
  file_path: &String,
  filename_prefix: &String,
  line: &String,
) -> Result<(), Error> {
  let date = Local::now().format("%Y%m%d_%H").to_string();
  let filename = &format!("{}_{}", filename_prefix, date);
  let path = Path::new(file_path).join(filename);
  let compound_path = path.to_str().expect(&format!(
    "Could not join file paths: {} and {}",
    &file_path, &filename
  ));

  let mut output_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(&compound_path)
    .expect(&format!("Could not open file \"{}\"", &compound_path));

  writeln!(output_file, "{}", line)
}
