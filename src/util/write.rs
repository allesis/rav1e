use serde::Serialize;
use std::io::Write;
use std::fs::File;

#[allow(dead_code)]
/// Allows writing a serializable object to a file
///
/// E.g. to_file(serializable_data, filename_str).unwrap();
pub fn to_file<T: Serialize>(
  data: T, filename: &str,
) -> Result<(), std::io::Error> {
  let mut file = match File::create(filename) {
    Ok(file) => file,
    Err(err) => return Err(err),
  };

  let data_string = match serde_json::to_string(&data) {
    Ok(data_string) => data_string,
    Err(err) => return Err(err.into()),
  };
  let data_bytes = data_string.as_bytes();
  match file.write_all(&data_bytes) {
    Ok(_) => Ok(()),
    Err(err) => Err(err),
  }
}
