#[macro_use]
pub mod hash_buffer;
pub mod util;
use std::{
  collections::{hash_map::DefaultHasher, HashMap},
  hash::{Hash, Hasher},
  sync::{Arc, Mutex, RwLock},
};

use num_traits::ToPrimitive;
use vq::{Quantizer, ScalarQuantizer};

use crate::{transform::TxSize, Pixel};

// NOTE: Change this to set the size of hashes used in coeff hashing
// TODO: These should probably be in hash/mod.rs or similar
pub type HashType = u16;
pub type HashMapType = HashMap<HashType, HashObject>;
pub type HashMapVecType = Arc<RwLock<[HashMapType; TxSize::TX_SIZES_ALL]>>;
pub type HashBufferType = Arc<Mutex<Vec<(HashType, HashObject, usize)>>>;
pub const HASHMASK: HashType = HashType::MAX;

pub struct HashObject {
  pub cul_level: u8,
  pub hash_coeffs: Vec<i32>,
}

pub fn quantize<T: Pixel>(coeffs: &[<T as Pixel>::Coeff]) -> Vec<u8> {
  let vec_coeffs =
    coeffs.iter().map(|coeff| coeff.to_f32().unwrap()).collect::<Vec<f32>>();

  let sq: ScalarQuantizer = ScalarQuantizer::new(0.0, 31.0, 32).unwrap();

  let qcoeffs = sq.quantize(&vec_coeffs).unwrap();

  qcoeffs
}

pub fn hashcoeffs<T: Pixel>(coeffs: Vec<u8>) -> HashType {
  let mut hasher = DefaultHasher::new();
  coeffs.iter().for_each(|coeff| (*coeff).hash(&mut hasher));
  let hash = hasher.finish();
  (hash & (HASHMASK as u64)).try_into().expect("FAILED TO CONVERT HASH")
}
