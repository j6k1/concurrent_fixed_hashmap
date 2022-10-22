use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::sync::{atomic, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::sync::atomic::AtomicUsize;

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
    buckets:Vec<(RwLock<Vec<(K,V)>>,AtomicUsize)>,
}
impl<K,V> ConcurrentFixedHashMap<K,V> where K: Hash + Eq {
    pub fn with_size(size:usize) -> ConcurrentFixedHashMap<K,V> {
        let mut buckets = Vec::with_capacity(size);
        buckets.resize_with(size,|| (RwLock::default(),AtomicUsize::new(0)));

        ConcurrentFixedHashMap {
            buckets:buckets,
        }
    }

    pub fn get<Q: ?Sized>(&self,k: &Q) -> Option<ReadGuard<'_,K,V>> where K: Borrow<Q>, Q: Hash + Eq + PartialEq<K> {
        let mut hasher = DefaultHasher::default();

        k.hash(&mut hasher);

        let index = (hasher.finish() % self.buckets.len() as u64) as usize;

        if self.buckets[index].1.load(atomic::Ordering::Acquire) == 0 {
            None
        } else {
            match self.buckets[index].0.read() {
                Ok(bucket) => {
                    for i in 0..bucket.len() {
                        if k == &bucket[i].0 {
                            return Some(ReadGuard::new(bucket, i));
                        }
                    }

                    None
                },
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }

    pub fn get_mut<Q: ?Sized>(&self,k: &Q) -> Option<WriteGuard<'_,K,V>> where K: Borrow<Q>, Q: Hash + Eq + PartialEq<K> {
        let mut hasher = DefaultHasher::default();

        k.hash(&mut hasher);

        let index = (hasher.finish() % self.buckets.len() as u64) as usize;

        if self.buckets[index].1.load(atomic::Ordering::Acquire) == 0 {
            None
        } else {
            match self.buckets[index].0.write() {
                Ok(bucket) => {
                    for i in 0..bucket.len() {
                        if k == &bucket[i].0 {
                            return Some(WriteGuard::new(bucket, i));
                        }
                    }

                    None
                },
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }

    pub fn insert(&self,k: K, value:V) -> Option<V> {
        let mut hasher = DefaultHasher::default();

        k.hash(&mut hasher);

        let index = (hasher.finish() % self.buckets.len() as u64) as usize;

        match self.buckets[index].0.write() {
            Ok(mut bucket) => {
                for i in 0..bucket.len() {
                    if k == bucket[i].0 {
                        return Some(mem::replace(&mut bucket[i].1,value));
                    }
                }

                self.buckets[(hasher.finish() % self.buckets.len() as u64) as usize].1.fetch_add(1,atomic::Ordering::Release);

                bucket.push((k,value));

                None
            },
            Err(e) => {
                panic!("{}",e);
            }
        }
    }

    pub fn insert_new(&self,k: K, value:V) {
        let mut hasher = DefaultHasher::default();

        k.hash(&mut hasher);

        let index = (hasher.finish() % self.buckets.len() as u64) as usize;

        match self.buckets[index].0.write() {
            Ok(mut bucket) => {
                for i in 0..bucket.len() {
                    if k == bucket[i].0 {
                        return;
                    }
                }

                self.buckets[(hasher.finish() % self.buckets.len() as u64) as usize].1.fetch_add(1,atomic::Ordering::Release);

                bucket.push((k,value));
            },
            Err(e) => {
                panic!("{}",e);
            }
        }
    }

    pub fn contains_key<Q: ?Sized>(&self,k: &Q) -> bool where K: Borrow<Q>, Q: Hash + Eq + PartialEq<K> {
        let mut hasher = DefaultHasher::default();

        k.hash(&mut hasher);

        let index = (hasher.finish() % self.buckets.len() as u64) as usize;

        if self.buckets[index].1.load(atomic::Ordering::Acquire) == 0 {
            false
        } else if self.buckets[index].1.load(atomic::Ordering::Acquire) == 1 {
            true
        } else {
            match self.buckets[index].0.read() {
                Ok(bucket) => {
                    for i in 0..bucket.len() {
                        if k == &bucket[i].0 {
                            return true;
                        }
                    }

                    return false;
                },
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }
}