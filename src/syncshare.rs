#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

// #[cfg(feature = "sync")]
/// Thread safe scoped shared state wrapper with ergonomic access methods
pub struct SyncShare<T> {
    inner: std::sync::RwLock<T>,
}

impl<T> SyncShare<T> {
    /// Creates a new 'SyncShare' wrapping the given value
    ///
    /// # Example:
    /// ```
    /// let shared = SyncShare::new(42)
    /// ```
    ///
    pub fn new(value: T) -> Self {
        Self {
            inner: RwLock::new(value),
        }
    }

    /// Provides immutable access to the inner value via a scoped closure
    ///
    /// # Panics
    /// Panics if the rwlock becomes poisoned
    ///
    /// # Example
    /// ```
    /// shared.with(|val| print!("{val}"));
    /// ```
    ///
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let guard = self.inner.read().unwrap();
        f(&*guard)
    }

    /// Provides immutable access to the inner value via a scoped closure
    ///
    /// #Panics
    /// Panics if the rwlock becomes poisoned
    ///
    /// # Example
    /// ```
    /// shared.with_mut(|val| *val += 1);
    /// ```
    ///
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.inner.write().unwrap();
        f(&mut *guard)
    }

    /// Attempts to provide immutable access. Returns 'None' if the lock is unavailable
    ///
    /// # Example
    /// ```
    /// if let Some(val) = shared.try_with(|v| *v) {
    ///     println!("value: {val}")
    /// }
    /// ```
    ///
    pub fn try_with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.inner.try_read().ok().map(|guard| f(&*guard))
    }

    /// Attempts to provide mutable access. Returns 'None' if the lock is unavailable
    ///
    /// # Example
    /// ```
    /// if let Some(_) = shared.try_with_mut(|v| v.push(10)) {
    ///     // successfully modified
    /// }
    /// ```
    ///
    pub fn try_with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        self.inner.try_write().ok().map(|mut guard| f(&mut *guard))
    }

    /// Clones and returns the inner value. Requires 'T: Clone'.
    ///
    /// # Example
    /// ```
    /// let snapshot = shared.snapshot();
    /// ```
    ///
    pub fn snapshot(&self) -> T
    where
        T: Clone,
    {
        self.with(|val| val.clone())
    }

    /// Replaces the inner value with a new one.
    ///
    /// # Example
    /// ```
    /// shared.replace(100)
    /// ```
    pub fn replace(&self, new: T) {
        self.with_mut(|val| *val = new);
    }

    /// Aquires an immutable borrow guard for the inner value
    ///
    /// # Panics
    /// Panics if the lock is poisoned
    ///
    /// # Example
    /// ```
    /// let guard = shared.borrow();
    /// println!("Value: {}", *guard);
    /// ```
    ///
    pub fn borrow(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read().unwrap()
    }

    /// Aquires a mutable borrow guard for the inner value
    ///
    /// # Panics
    /// Panics if the lock is poisoned
    ///
    /// # Example
    /// ```
    /// let mut guard = shared.borrow_mut();
    /// *guard += 1;
    /// ```
    ///
    pub fn borrow_mut(&self) -> RwLockWriteGuard<'_, T> {
        self.inner.write().unwrap()
    }
}

#[cfg(feature = "serde")]
impl<T: Serialize> Serialize for SyncShare<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.with(|val| val.serialize(serializer))
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de>> Deserialize<'de> for SyncShare<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(SyncShare::new)
    }
}
