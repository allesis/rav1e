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

    temp_buffer_lock.drain(0..).for_each(|(hash, value, tx, bsize)| {
      hash_buffer_lock.push((hash, value, tx, bsize));
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

pub(crate) use commit;
pub(crate) use optionize_buffer;
pub(crate) use rollback;
