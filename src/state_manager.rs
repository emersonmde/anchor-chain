use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Clone)]
pub struct StateManager<K, V> {
    inner: Arc<RwLock<HashMap<K, V>>>,
}

impl<K: Eq + Hash + Clone, V: Clone> StateManager<K, V> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        self.inner.read().await.get(key).cloned()
    }

    pub async fn insert(&self, key: K, value: V) {
        self.inner.write().await.insert(key, value);
    }

    pub async fn remove(&self, key: &K) -> Option<V> {
        self.inner.write().await.remove(key)
    }

    pub async fn len(&self) -> usize {
        self.inner.read().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.inner.read().await.is_empty()
    }

    pub async fn clear(&self) {
        self.inner.write().await.clear()
    }

    pub async fn contains_key(&self, key: &K) -> bool {
        self.inner.read().await.contains_key(key)
    }

    pub async fn keys(&self) -> Vec<K> {
        self.inner.read().await.keys().cloned().collect()
    }

    pub async fn values(&self) -> Vec<V> {
        self.inner.read().await.values().cloned().collect()
    }

    pub async fn read(&self) -> RwLockReadGuard<HashMap<K, V>> {
        self.inner.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<HashMap<K, V>> {
        self.inner.write().await
    }
}

impl<K: Eq + Hash + Clone, V: Clone> StateManager<K, Vec<V>> {
    pub async fn push(&self, key: K, value: V) {
        let mut map = self.inner.write().await;
        map.entry(key)
            .and_modify(|vec| vec.push(value.clone()))
            .or_insert(vec![value]);
    }
}

impl<K: Eq + Hash + Clone, V: Clone> Default for StateManager<K, V> {
    fn default() -> Self {
        StateManager::<K, V>::new()
    }
}
