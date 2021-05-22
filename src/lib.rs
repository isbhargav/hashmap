use std::{collections::hash_map::DefaultHasher, hash::Hasher, usize};
use std::{hash::Hash, mem};

const INITAL_NBUCKETS: usize = 1;
struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}
impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + Copy, // we just need constrain in key to be hashable
{
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
    fn hash(key: K, buckets_len: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket = (hasher.finish() % buckets_len as u64) as usize;
        return bucket;
    }
    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITAL_NBUCKETS,
            n => 2 * n,
        };
        let mut new_bucktes = Vec::with_capacity(target_size);
        new_bucktes.extend((0..target_size).map(|_| Vec::new()));

        for (key, val) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            // let mut hasher = DefaultHasher::new();
            // key.hash(&mut hasher);
            // let bucket = (hasher.finish() % new_bucktes.len() as u64) as usize;
            let bucket = Self::hash(key, new_bucktes.len());
            new_bucktes[bucket].push((key, val));
        }
        let _ = mem::replace(&mut self.buckets, new_bucktes);
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items < 3 * self.buckets.len() / 4 {
            self.resize();
        }
        // let mut hasher = DefaultHasher::new();
        // key.hash(&mut hasher);
        // let idx = (hasher.finish() % self.buckets.len() as u64) as usize;
        let bucket = Self::hash(key, self.buckets.len());
        let bucket = &mut self.buckets[bucket];
        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        let mut map = HashMap::new();
        map.insert("test", 42);
    }
}