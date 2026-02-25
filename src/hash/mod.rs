#[macro_use]
pub mod hash_buffer;
pub mod hashframe;
pub mod util;
use std::{
  collections::{HashMap, hash_map::DefaultHasher},
  hash::{Hash, Hasher},
  sync::{Arc, Mutex, RwLock},
};

use num_traits::ToPrimitive;

use crate::{Pixel, prelude::TxType, transform::TxSize};

// NOTE: Change this to set the size of hashes used in coeff hashing
// TODO: These should probably be in hash/mod.rs or similar
pub type HashType = u16;
pub type HashMapType = HashMap<HashType, HashObject>;
pub type HashMapVecType =
  Arc<RwLock<[[HashMapType; TxSize::TX_SIZES_ALL]; 3]>>;
pub type HashBufferType =
  Arc<Mutex<Vec<(HashType, HashObject, usize, usize)>>>;
pub const HASHMASK: HashType = HashType::MAX;

pub struct HashObject {
  pub cul_level: u8,
  pub eob: u16,
  pub hash_coeffs: Vec<u16>,
  pub tx_type: TxType,
  pub tx_size: TxSize,
}

pub fn old_hashcoeffs<T: Pixel>(
  coeffs: &mut [<T as Pixel>::Coeff], eob: u16,
) -> HashType {
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
  let hash = hasher.finish();
  (hash & (HASHMASK as u64)).try_into().expect("FAILED TO CONVERT HASH")
}

#[inline(always)]
pub fn hashcoeffs<T: Pixel>(
  coeffs: &mut [<T as Pixel>::Coeff], _eob: u16,
) -> HashType {
  let count: HashType =
    (coeffs.iter().filter(|x| x.to_i32().unwrap() != 0).count() as HashType
      & HashType::MAX)
      .into();
  let magnitude: HashType =
    (coeffs.iter().map(|x| x.to_i32().unwrap()).sum::<i32>() as HashType
      & HashType::MAX)
      .into();

  ((count << 6) & 0xFFC0) & (magnitude & 0x003F)
}
