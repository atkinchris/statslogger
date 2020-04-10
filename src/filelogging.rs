use chrono::Local;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn write_to_file(
  file_path: &String,
  filename_prefix: &String,
  line: &String,
) -> Result<(), String> {
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
    .or_else(|err| {
      Err(format!(
        "Could not create file \"{}\": {}",
        &compound_path, err
      ))
    })?;

  writeln!(output_file, "{}", line).or_else(|err| {
    Err(format!(
      "Could not write to file \"{}\": {}",
      &compound_path, err
    ))
  })
}
