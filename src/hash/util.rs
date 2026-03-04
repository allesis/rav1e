use v_frame::pixel::Pixel;

use super::{HashBufferType, HashMapVecType, HashObject, HashType};
use crate::ec::Writer;

#[inline(always)]
pub fn write_hash<W: Writer>(w: &mut W, hash: HashType) {
  const HASH_BITS: u32 = HashType::BITS;

  ensure_sizing!(HASH_BITS, u8::MAX);

  w.literal(HASH_BITS as u8, hash as u32);
}

#[inline(always)]
pub fn get_hash_object<T: Pixel>(
  hashmap: HashMapVecType, hash: HashType,
) -> (u16, u8, Vec<i32>) {
  // NOTE: This could either be a lock or a try_lock
  // If a lock is used, we will wait until the hashmap is available to continue
  // which may DESTROY performance
  // If we use a try_lock, we may miss chances to decrease encoding size
  // For now a lock will be used
  let hashmaps_lock = hashmap.read().expect("FAILED TO LOCK HASHMAP");
  if let Some(hashmap_lock) = hashmaps_lock.get(0) {
    if let Some(hash_object) = hashmap_lock.get(&hash) {
      return (0, hash_object.cul_level, hash_object.hash_coeffs.clone());
    }
  }
  return (1, 0, vec![]);
}

#[inline(always)]
pub fn add_hash_object<T: Pixel>(
  hash_buffer: Option<HashBufferType>, cul_lvl: u8, hash: HashType,
  tx_size: usize, rcoeffs: &mut [<T as Pixel>::Coeff],
) {
  if let Some(hash_buffer) = hash_buffer {
    let mut hash_buffer_lock =
      hash_buffer.lock().expect("FAILED TO LOCK HASHMAP");
    let hash_object = HashObject {
      cul_level: cul_lvl,
      hash_coeffs: rcoeffs.iter().map(|&e| e.into()).collect(),
    };
    hash_buffer_lock.push((hash, hash_object, tx_size));
  }
}

#[inline(always)]
pub fn add_hashs_to_map<T: Pixel>(
  hashmap: HashMapVecType, hash_buffer: HashBufferType,
) {
  let mut hashmap_lock = hashmap.write().expect("FAILED TO LOCK HASHMAP");

  let mut hash_buffer_lock =
    hash_buffer.lock().expect("FAILED TO LOCK NEW HASHMAP");

  hash_buffer_lock.iter().for_each(|v| {
    let (hash, value, tx) = v;

    let hashmap = hashmap_lock.get_mut(*tx).expect("BAD INDEX ON TX SIZE");

    hashmap.insert(
      *hash,
      HashObject {
        cul_level: value.cul_level,
        hash_coeffs: value.hash_coeffs.clone(),
      },
    );
  });
  *hash_buffer_lock = Vec::new();
}
