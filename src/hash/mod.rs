pub mod hashframe;
use num_traits::ToPrimitive;

use crate::Pixel;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hashcoeffs<T: Pixel>(
  coeffs: &mut [<T as Pixel>::Coeff], eob: u16, width: usize, height: usize,
) -> u64 {
  let mut hasher = DefaultHasher::new();
  coeffs.iter().for_each(|coeff| {
    if coeff.to_i32().unwrap() == 0 {
    } else {
      coeff.to_i32().unwrap().hash(&mut hasher)
    }
  });
  if eob == 0 {
    eob.hash(&mut hasher);
  } else {
    // WARN: Will never subtract with overflow since eob > 0
    (eob - 1).hash(&mut hasher);
  }
  width.hash(&mut hasher);
  height.hash(&mut hasher);
  let hash = hasher.finish();
  hash
}
