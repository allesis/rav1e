use serde::Serialize;
use std::fs::File;
use std::io::{Error, Write};


#[allow(dead_code)]
pub fn to_file<T: Serialize>(data: T, filename: &str) -> Result<(), Error> {
  let mut file = match File::create(filename) {
    Ok(file) => file,
    Err(err) => return Err(err),
  };

  let data_string = match serde_json::to_string(&data) {
    Ok(data_string) => data_string,
    Err(err) => return Err(err.into()),
  };

  let data_bytes = data_string.as_bytes();

  return match file.write_all(data_bytes) {
    Ok(_) => Ok(()),
    Err(err) => Err(err),
  }
}
