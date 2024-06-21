use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Clone)]
pub struct StateManager<T> {
    inner: Arc<RwLock<Vec<T>>>,
}

impl<T: Clone> StateManager<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn get(&self, index: usize) -> Option<T> {
        self.inner.read().await.get(index).cloned()
    }

    pub async fn push(&self, value: T) {
        self.inner.write().await.push(value)
    }

    pub async fn remove(&self, index: usize) -> T {
        self.inner.write().await.remove(index)
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

    pub async fn read(&self) -> RwLockReadGuard<Vec<T>> {
        self.inner.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<Vec<T>> {
        self.inner.write().await
    }
}

impl<T: Clone> Default for StateManager<T> {
    fn default() -> Self {
        StateManager::<T>::new()
    }
}
