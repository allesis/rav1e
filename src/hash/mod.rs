pub mod hashframe;
pub mod hashsegment;
use num_traits::ToPrimitive;

use crate::Pixel;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hashcoeffs<T: Pixel>(coeffs: &mut [<T as Pixel>::Coeff]) -> u64 {
  let mut hasher = DefaultHasher::new();
  coeffs.iter().for_each(|coeff| coeff.to_u64().hash(&mut hasher));
  let hash = hasher.finish();
  hash
}
