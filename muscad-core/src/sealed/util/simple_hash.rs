use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
use std::ops::BitXor;

#[cfg(target_pointer_width = "32")]
const MASK: usize = 0x9e_37_79_b9;
#[cfg(target_pointer_width = "64")]
const MASK: usize = 0x51_7c_c1_b7_27_22_0a_95;

pub type SimpleHashMap<K, V> = HashMap<K, V, BuildHasherDefault<NaiveHasher>>;
pub type SimpleHashSet<K> = HashSet<K, BuildHasherDefault<NaiveHasher>>;

#[derive(Debug, Clone)]
pub struct NaiveHasher {
    v: usize,
}

impl std::default::Default for NaiveHasher {
    #[inline]
    fn default() -> Self {
        Self { v: 0 }
    }
}

impl Hasher for NaiveHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.v = self.v.rotate_left(5).bitxor(*b as usize).wrapping_mul(MASK);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.v as u64
    }
}
