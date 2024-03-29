use std::{collections::hash_map::DefaultHasher, hash::Hasher, usize};
use std::{hash::Hash, mem};

const INITAL_NBUCKETS: usize = 1;
struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}
impl<K, V> HashMap<K, V>
where
    K: Hash + Eq + Copy, // we just need constrain on key to be hashable
    V: Copy,
{
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
    fn hash(key: &K, buckets_len: usize) -> usize {
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
            let bucket = Self::hash(&key, new_bucktes.len());
            new_bucktes[bucket].push((key, val));
        }
        let _ = mem::replace(&mut self.buckets, new_bucktes);
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items < 3 * self.buckets.len() / 4 {
            self.resize();
        }
        let bucket = Self::hash(&key, self.buckets.len());
        let bucket = &mut self.buckets[bucket];
        // for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
        // for and if uses pattern matching
        for item in bucket.iter_mut() {
            let (ref ekey, ref mut evalue) = item;
            if ekey == &key {
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));
        self.items += 1;
        return None;
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket_idx = Self::hash(&key, self.buckets.len());
        self.buckets[bucket_idx]
            .iter()
            .find(|(ref ekey, _)| &ekey == &key)
            .map(|(_, ref val)| val)
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_idx = Self::hash(&key, self.buckets.len());
        let item_idx = self.buckets[bucket_idx]
            .iter()
            .position(|&(ref k, _)| return k == key)?; //predicate uses pattern matching
        let remove = self.buckets[bucket_idx].remove(item_idx);
        self.items -= 1;
        return Some(remove.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        let val = 42;
        let key = "test";
        map.insert(key, val);
        assert_eq!(
            map.items, 1,
            "Test Failed for number of items {} == {}",
            map.items, 1
        );
    }
    #[test]
    fn get() {
        let mut map = HashMap::new();
        let val = 42;
        let key = "test";
        map.insert(key, val);
        if let Some(ret_val) = map.get(&"test") {
            // print!("{}",val);
            assert_eq!(
                ret_val, &val,
                "Test Failed for get method: {} == {}",
                ret_val, val
            );
        }
    }
    #[test]
    fn remove() {
        let mut map = HashMap::new();
        let val = 42;
        let key = "test";
        map.insert(key, val);
        if let Some(ret_val) = map.remove(&"test") {
            // print!("{}",val);
            assert_eq!(
                ret_val, val,
                "Test Failed for remove method: {} == {}",
                ret_val, val
            );
            assert_eq!(
                map.items, 0,
                "Test Failed for number of items after removal {} == {}",
                map.items, 0
            );
        }
    }
}
