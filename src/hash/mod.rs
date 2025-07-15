pub mod hashframe;
use num_traits::ToPrimitive;

use crate::Pixel;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hashcoeffs<T: Pixel>(
  coeffs: &mut [<T as Pixel>::Coeff], eob: u16, x: usize, y: usize, p: usize,
  width: usize, height: usize,
) -> u64 {
  let mut hasher = DefaultHasher::new();
  coeffs.iter().for_each(|coeff| coeff.to_u64().hash(&mut hasher));
  eob.hash(&mut hasher);
  x.hash(&mut hasher);
  y.hash(&mut hasher);
  p.hash(&mut hasher);
  width.hash(&mut hasher);
  height.hash(&mut hasher);
  let hash = hasher.finish();
  //println!("{}", hash);
  hash
}
