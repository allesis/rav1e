use super::HashBufferType;

pub struct HashBuffer {
  hash_buffer: HashBufferType,
  temp_buffer: HashBufferType,
}

impl HashBuffer {
  pub fn new(hash_buffer: Option<HashBufferType>) -> Option<Self> {
    if let Some(hash_buffer) = hash_buffer {
      Some(HashBuffer { hash_buffer, temp_buffer: HashBufferType::default() })
    } else {
      None
    }
  }

  pub fn commit(&self) {
    let mut hash_buffer_lock =
      self.hash_buffer.lock().expect("Failed to lock hash buffer");

    let mut temp_buffer_lock =
      self.temp_buffer.lock().expect("Failed to lock temp buffer");

    temp_buffer_lock.drain(0..).for_each(|(hash, value, tx)| {
      hash_buffer_lock.push((hash, value, tx));
    });
  }

  pub fn rollback(&self) {
    let mut temp_buffer_lock =
      self.temp_buffer.lock().expect("Failed to lock temp buffer");
    temp_buffer_lock.clear();
  }

  pub fn get_buffer(&self) -> HashBufferType {
    return self.temp_buffer.clone();
  }
}

impl Default for HashBuffer {
  fn default() -> HashBuffer {
    HashBuffer {
      hash_buffer: HashBufferType::default(),
      temp_buffer: HashBufferType::default(),
    }
  }
}

// TODO: These do not need to be macros
// The golden rule applies:
// 'Never do with a macro what you can do with a function'
macro_rules! optionize_buffer {
  ($hash_buffer:expr) => {{
    if let Some(ref hash_buffer) = $hash_buffer {
      Some(hash_buffer.get_buffer())
    } else {
      None
    }
  }};
}

macro_rules! rollback {
  ($hash_buffer:expr) => {{
    if let Some(ref hash_buffer) = $hash_buffer {
      hash_buffer.rollback();
    };
  }};
}

macro_rules! commit {
  ($hash_buffer:expr) => {{
    if let Some(ref hash_buffer) = $hash_buffer {
      hash_buffer.commit();
    };
  }};
}

// Ensure that `$test_size` is less than `$max_size`
// We could also turn this into a `const fn` as follows:
//
// ```
// const fn ensure_sizing_fn(test_size: usize, max_size: usize) {
//   assert!(test_size <= max_size);
// }
// ```
//
// However, this means that we don't get LSP warnings from static analysis
// So we use a macro to ensure we do
macro_rules! ensure_sizing {
  ($test_size:expr, $max_size:expr) => {{
    // This will allow us to catch bad sizing match ups before runtime
    // We need the `const _: () =` here or the assert! is not evaluated
    // until runtime
    // Which is not what we want
    const _: () = assert!($test_size as usize <= $max_size as usize);
  }};
}

pub(crate) use commit;
pub(crate) use ensure_sizing;
pub(crate) use optionize_buffer;
pub(crate) use rollback;
