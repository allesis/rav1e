use std::hash::{Hash, Hasher};
use v_frame::frame::Frame;
pub struct HashFrame(Frame);
impl<T: Pixel> Hash for HashFrame<T> {
fn hash<H: Hasher>(&self, state: &mut H) {
    }
}
