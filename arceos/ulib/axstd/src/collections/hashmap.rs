#![no_std]

// 启用堆分配
extern crate alloc;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};

// 假设有一个全局可用的 random() 函数返回 u128
extern "C" {
    fn random() -> u128;
}

// 自定义哈希器，使用 u128 随机种子
struct CustomHasher {
    seed: u64,
}

impl CustomHasher {
    fn new() -> Self {
        // 将 u128 随机数转换为 u64 种子
        let r = unsafe { random() };
        Self {
            seed: r as u64 ^ (r >> 64) as u64, // 混合高位和低位
        }
    }
}

impl Hasher for CustomHasher {
    fn write(&mut self, bytes: &[u8]) {
        // 改进的哈希混合，利用更多随机性
        for &byte in bytes {
            self.seed = self.seed.wrapping_mul(0x85EBCA77B2DE4D5F).wrapping_add(byte as u64);
        }
    }

    fn finish(&self) -> u64 {
        self.seed
    }
}

// 哈希表条目
struct Entry<K, V> {
    key: K,
    value: V,
    hash: u64,
}

// 哈希表实现
pub struct HashMap<K, V> {
    buckets: Vec<Vec<Entry<K, V>>>,
    size: usize,
}
pub struct Keys<'a, K, V> {
    inner: Iter<'a, K, V>  // 复用Iter的实现
}

pub struct Values<'a, K, V> {
    inner: Iter<'a, K, V>  // 复用Iter的实现
}
impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(1);
        buckets.push(Vec::new()); 
        Self {
            buckets,
            size: 0,
        }
    }

    // 计算键的哈希值
    fn hash_key(&self, key: &K) -> u64 {
        let mut hasher = CustomHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    // 插入键值对
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.size >= self.buckets.len() * 3 / 4 {
            self.resize();
        }

        let hash = self.hash_key(&key);
        let bucket_index = (hash as usize) % self.buckets.len();

        // 检查是否已存在
        for entry in &mut self.buckets[bucket_index] {
            if entry.key == key {
                return Some(core::mem::replace(&mut entry.value, value));
            }
        }
        // 安全插入：总是追加到链表末尾
        self.buckets[bucket_index].push(Entry { key, value, hash });
        self.size += 1;
        None
    }

    // 获取值
    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash_key(key);
        let bucket_index = (hash as usize) % self.buckets.len();
    
        // 线性搜索链表
        self.buckets[bucket_index]
            .iter()
            .find(|entry| entry.key == *key)
            .map(|entry| &entry.value)
    }

    fn resize(&mut self) {
        let new_capacity = self.buckets.len() * 2; // 通常翻倍
        let mut new_buckets = Vec::with_capacity(new_capacity);
        
        // 初始化新桶
        new_buckets.resize_with(new_capacity, Vec::new);
        
        // 重哈希所有元素（总是追加到链表尾部）
        for bucket in self.buckets.drain(..) {
            for entry in bucket {
                let new_bucket_index = entry.hash as usize % new_capacity;
                new_buckets[new_bucket_index].push(entry); // 总是 push_back
            }
        }
        
        self.buckets = new_buckets;
    }
    // 实现迭代器方法
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            outer: self.buckets.iter(),
            inner: None,
        }
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys { inner: self.iter() }
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values { inner: self.iter() }
    }

}
// 键值对迭代器.
pub struct Iter<'a, K, V> {
    outer: core::slice::Iter<'a, Vec<Entry<K, V>>>,
    inner: Option<core::slice::Iter<'a, Entry<K, V>>>,
}
// 实现Iterator trait for Iter
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.inner {
                if let Some(entry) = inner.next() {
                    return Some((&entry.key, &entry.value));
                }
            }

            // 当前桶已遍历完，移动到下一个桶
            match self.outer.next() {
                Some(bucket) => self.inner = Some(bucket.iter()),
                None => return None,
            }
        }
    }
}
