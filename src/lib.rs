use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct ReadGuard<'a,K,V> {
    locked_bucket:RwLockReadGuard<'a, Vec<(K,V)>>,
    index:usize
}
impl<'a,K,V> ReadGuard<'a,K,V> {
    fn new(locked_bucket:RwLockReadGuard<'a,Vec<(K,V)>>,index:usize) -> ReadGuard<'a,K,V> {
        ReadGuard {
            locked_bucket:locked_bucket,
            index:index
        }
    }
}
impl<'a,K,V> Deref for ReadGuard<'a,K,V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.locked_bucket.deref().index(self.index).1
    }
}
pub struct WriteGuard<'a,K,V> {
    locked_bucket:RwLockWriteGuard<'a, Vec<(K,V)>>,
    index:usize
}
impl<'a,K,V> WriteGuard<'a,K,V> {
    fn new(locked_bucket:RwLockWriteGuard<'a,Vec<(K,V)>>,index:usize) -> WriteGuard<'a,K,V> {
        WriteGuard {
            locked_bucket:locked_bucket,
            index:index
        }
    }
}
impl<'a,K,V> Deref for WriteGuard<'a,K,V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.locked_bucket.deref().index(self.index).1
    }
}

impl<'a,K,V> DerefMut for WriteGuard<'a,K,V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.locked_bucket.deref_mut().index_mut(self.index).1
    }
}
pub struct ConcurrentFixedHashMap<K,V> where K: Hash + Eq {
    buckets:Vec<RwLock<Vec<(K,V)>>>,
}
impl<K,V> ConcurrentFixedHashMap<K,V> where K: Hash + Eq {
    pub fn with_size(size:usize) -> ConcurrentFixedHashMap<K,V> {
        let mut buckets = Vec::with_capacity(size);
        buckets.resize_with(size,RwLock::default);

        ConcurrentFixedHashMap {
            buckets:buckets,
        }
    }

    pub fn get<Q: ?Sized>(&self,k: &Q) -> Option<ReadGuard<'_,K,V>> where K: Borrow<Q>, Q: Hash + Eq + PartialEq<K> {
        let mut hasher = DefaultHasher::default();

        k.hash(&mut hasher);

        match self.buckets[hasher.finish() as usize % self.buckets.len()].read() {
            Ok(bucket) => {
                for i in 0..bucket.len() {
                    if k == &bucket[i].0 {
                        return Some(ReadGuard::new(bucket,i));
                    }
                }

                None
            },
            Err(e) => {
                panic!("{}",e);
            }
        }
    }
}