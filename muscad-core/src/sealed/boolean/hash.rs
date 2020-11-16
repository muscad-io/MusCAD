use super::*;

use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;

pub type F<T> = EntityHashKey<FaceRef<T>>;
pub type E<T> = EntityHashKey<EdgeRef<T>>;
pub type V<T> = EntityHashKey<VertexRef<T>>;
pub type VV<T> = (V<T>, V<T>);
pub type Map<K, V> = SimpleHashMap<K, V>;
pub type Set<T> = SimpleHashSet<T>;

pub fn reverse_pair<T: Clone>(t: &(T, T)) -> (T, T) {
    (t.1.clone(), t.0.clone())
}

pub fn new_set<K>() -> Set<K> {
    HashSet::default()
}

pub fn new_map<K, V>() -> Map<K, V> {
    HashMap::default()
}

#[allow(unused)]
pub fn new_map_with_capacity<K, V>(n: usize) -> Map<K, V> {
    HashMap::with_capacity_and_hasher(n, BuildHasherDefault::<NaiveHasher>::default())
}
