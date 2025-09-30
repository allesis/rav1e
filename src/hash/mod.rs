pub mod hashframe;
use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

use num_traits::ToPrimitive;

use crate::{Pixel, prelude::TxType};

pub fn hashcoeffs<T: Pixel>(
  coeffs: &mut [<T as Pixel>::Coeff], eob: u16, tx_type: TxType, width: usize,
  height: usize,
) -> u32 {
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
  (tx_type as usize).hash(&mut hasher);
  width.hash(&mut hasher);
  height.hash(&mut hasher);
  let hash = hasher.finish();
  (((hash >> 32) ^ hash) & 0x00000000FFFFFFFF)
    .try_into()
    .expect("FAILED TO CONVERT HASH")
}
