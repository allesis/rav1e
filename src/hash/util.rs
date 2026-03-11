use v_frame::pixel::{CastFromPrimitive, Pixel};

use super::{HashBufferType, HashMapVecType, HashObject, HashType};
use crate::ec::Writer;

#[inline(always)]
pub fn write_hash<W: Writer>(w: &mut W, hash: HashType) {
  const HASH_BITS: u32 = HashType::BITS;

  ensure_sizing!(HASH_BITS, u8::MAX);

  for byte in hash.to_be_bytes() {
    w.bit(((byte >> 7) & 0b1).into());
    w.bit(((byte >> 6) & 0b1).into());
    w.bit(((byte >> 5) & 0b1).into());
    w.bit(((byte >> 4) & 0b1).into());
    w.bit(((byte >> 3) & 0b1).into());
    w.bit(((byte >> 2) & 0b1).into());
    w.bit(((byte >> 1) & 0b1).into());
    w.bit(((byte >> 0) & 0b1).into());
  }
  //w.literal(HASH_BITS as u8, hash as u32);
}

#[inline(always)]
pub fn get_hash_object<T: Pixel>(
  hashmap: HashMapVecType, hash: HashType,
) -> Option<HashObject> {
  // NOTE: This could either be a lock or a try_lock
  // If a lock is used, we will wait until the hashmap is available to continue
  // which may DESTROY performance
  // If we use a try_lock, we may miss chances to decrease encoding size
  // For now a lock will be used
  let hashmap_lock = hashmap.read().expect("FAILED TO LOCK HASHMAP");
  if let Some(hash_object) = hashmap_lock.get(&hash) {
    return Some(hash_object.clone());
  }
  return None;
}

#[inline(always)]
pub fn add_hash_object<T: Pixel>(
  hash_buffer: Option<HashBufferType>, cul_lvl: u8, eob: u16, hash: HashType,
  rcoeffs: &mut [<T as Pixel>::Coeff],
) {
  if hash == 44297 {
    dbg!(&rcoeffs);
  }
  if let Some(hash_buffer) = hash_buffer {
    let mut hash_buffer_lock =
      hash_buffer.lock().expect("FAILED TO LOCK HASHMAP");
    let hash_object = HashObject {
      cul_level: cul_lvl,
      hash_coeffs: rcoeffs.iter().map(|&e| e.into()).collect(),
      hash_eob: eob,
    };
    hash_buffer_lock.push((hash, hash_object));
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
    let (hash, value) = v;

    hashmap_lock.insert(
      *hash,
      HashObject {
        cul_level: value.cul_level,
        hash_coeffs: value.hash_coeffs.clone(),
        hash_eob: value.hash_eob,
      },
    );
  });
  *hash_buffer_lock = Vec::new();
}

pub fn replace_coeffs<T: Pixel>(from: &[i32], to: &mut [<T as Pixel>::Coeff]) {
  for (r, c) in to.iter_mut().zip(from.iter().map(|&c| i32::cast_from(c))) {
    *r = T::Coeff::cast_from(c);
  }
}
