#[macro_use]
pub mod hash_buffer;
pub mod util;
use std::{
  collections::{hash_map::DefaultHasher, HashMap},
  hash::{Hash, Hasher},
  sync::{Arc, Mutex, RwLock},
};

use num_traits::ToPrimitive;

use crate::Pixel;

// NOTE: Change this to set the size of hashes used in coeff hashing
// TODO: These should probably be in hash/mod.rs or similar
pub type HashType = u16;
pub type CoeffVecType = Vec<i32>;
pub type HashMapType = HashMap<HashType, HashObject>;
pub type HashMapVecType = Arc<RwLock<HashMapType>>;
pub type HashBufferType = Arc<Mutex<Vec<(HashType, HashObject)>>>;
pub const HASHMASK: HashType = HashType::MAX;

#[derive(Clone)]
pub struct HashObject {
  pub cul_level: u8,
  pub hash_coeffs: Vec<i32>,
  pub hash_eob: u16,
}

pub fn quantize<T: Pixel>(coeffs: &mut [<T as Pixel>::Coeff]) -> CoeffVecType {
  coeffs
    .iter()
    // TODO: Find a better quantization method
    .map(|coeff| ((*coeff).to_i32().unwrap()) >> 3)
    .collect::<CoeffVecType>()
}

pub fn hashcoeffs<T: Pixel>(coeffs: CoeffVecType) -> HashType {
  let mut hasher = DefaultHasher::new();
  coeffs.iter().for_each(|coeff| {
    if *coeff != 0 {
      (*coeff).hash(&mut hasher)
    }
  });
  let hash = hasher.finish();
  (hash & (HASHMASK as u64)).try_into().expect("FAILED TO CONVERT HASH")
}
