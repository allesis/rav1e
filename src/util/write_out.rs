use serde::Serialize;
use std::fs::File;
pub fn to_file<T: Serializable>(
  data: T, filename: &str,
) -> Result<(), std::io::Error> {
  let file = match File::create(filename) {
    Ok(file) => file,
    Err(err) => return err,
  };

  let data_bytes = match serde_json::to_string(&data) {
    Ok(data_string) => data_string.as_bytes(),
    Err(err) => return err,
  };
  match file.write_all(&data_bytes) {
    Ok(_) => Ok(()),
    Err(err) => err,
  }
}
