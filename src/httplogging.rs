use reqwest::blocking::Client;

pub fn post_to_url(url: &String, body: String) -> Result<(), String> {
  let client = Client::new();
  let response = client
    .post(url)
    .json(&body)
    .send()
    .or_else(|err| Err(format!("Error posting to url: {}", err)))?;

  response
    .error_for_status()
    .and_then(|_| Ok(()))
    .or_else(|err| {
      Err(format!(
        "Error status from url: {}",
        err.status().expect("Error unwrapping post status")
      ))
    })
}
