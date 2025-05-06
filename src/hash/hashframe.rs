use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use v_frame::prelude::Pixel;
use v_frame::frame::Frame;
#[derive(Debug, Serialize, Deserialize)]
pub struct HashFrame<T: Pixel>(Frame<T>);

impl<T: Pixel> HashFrame<T> {
  fn new(frame: Frame<T>) -> Self {
    Self(frame)
  }
}
impl<T: Pixel> From<Frame<T>> for HashFrame<T> {
  fn from(value: Frame<T>) -> Self {
    Self(value)
  }
}
impl<T: Pixel> Into<Frame<T>> for HashFrame<T> {
  fn into(self) -> Frame<T> {
    self.0
  }
}
impl<T: Pixel> Hash for HashFrame<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.planes.iter().for_each(|plane| plane.iter().for_each(|pixel| pixel.to_u8().hash(state)));
  }
}
