use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use v_frame::prelude::Pixel;
use v_frame::frame::Frame;
#[derive(Debug)]
pub struct HashFrame<T: Pixel>(Frame<T>);
impl<T: Pixel> Hash for HashFrame<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    todo!();
  }
}

impl<T: Pixel> Serialize for HashFrame<T> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer {
      todo!();
  }
}
impl<'de, T: Pixel> Deserialize<'de> for HashFrame<T> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de> {
      todo!();
  }
}
