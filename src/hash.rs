use sha2::digest::Digest;
use sha2::Sha256;
use std::string::String;

pub fn hash_string(input: String) -> String {
  format!("{:x}", Sha256::digest(input.as_bytes()))[..8].to_owned()
}
