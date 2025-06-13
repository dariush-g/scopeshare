use core::fmt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cell::{Ref, RefCell, RefMut};
// Scoped, single-threaded shared access wrapper using 'RefCell<T>',
//
// Allows mutable or immutable access to a value within controlled scopes
// Panics if borrow rules are violated at runtime
pub struct ScopeShare<T> {
    inner: RefCell<T>,
}

impl<T> ScopeShare<T> {
    /// Creates a new 'ScopeShare' wrapping the given value.
    ///
    /// # Example
    /// '''
    /// let shared = ScopeShare::new(42);
    /// '''
    ///
    pub fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    /// Provides immutable access via a scoped closure
    ///
    /// # Panics
    /// Panics at runtime if a mutable borrow occurs
    ///
    /// # Example
    /// '''
    /// shared.with(|val| print!("{val}"))
    /// '''
    ///
    #[track_caller]
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let borrow = self.inner.borrow();
        f(&borrow)
    }

    /// Provides mutable access via a scoped closure
    ///
    /// # Panics
    /// Panics at runtime if another borrow is active
    ///
    /// # Example
    /// '''
    /// shared.with_mut(|val| *val += 1);
    /// '''
    ///
    #[track_caller]
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut borrow = self.inner.borrow_mut();
        f(&mut borrow)
    }

    /// Aquires a mutable borrow of the inner value.
    ///
    /// Returns a guard that implements 'DerefMut'.
    ///
    /// # Panics
    /// Panics at runtime if any other borrow is currently active.
    ///
    /// # Example
    /// '''
    /// let shared = ScopeShare::new(42);
    /// let mut guard = shared.borrow_mut();
    /// *guard += 1;
    /// '''
    ///
    #[track_caller]
    pub fn borrow_mut(&self) -> ScopeRefMut<'_, T> {
        ScopeRefMut {
            inner: self.inner.borrow_mut(),
        }
    }
    /// Aquires an immutable borrow of the inner value.
    ///
    /// Returns a guard that implements 'Deref<Target = T>.
    ///
    /// # Panics
    /// Panics at runtime if a mutable borrow is already active
    ///
    /// # Example
    /// '''
    /// let shared = ScopeShare::new(42);
    /// let guard = shared.borrow();
    /// println!("Value: {}", *guard);
    /// '''
    ///
    #[track_caller]
    pub fn borrow(&self) -> ScopeRef<'_, T> {
        ScopeRef {
            inner: self.inner.borrow(),
        }
    }
}

// This clones the inner data
impl<T: Clone> Clone for ScopeShare<T> {
    fn clone(&self) -> Self {
        ScopeShare::new(self.inner.borrow().clone())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ScopeShare<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScopeShare")
            .field("value", &self.inner.borrow())
            .finish()
    }
}
#[cfg(feature = "serde")]
impl<T: Serialize> Serialize for ScopeShare<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.borrow().serialize(serializer)
    }
}
#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de>> Deserialize<'de> for ScopeShare<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(ScopeShare::new)
    }
}

pub struct ScopeRef<'a, T> {
    inner: Ref<'a, T>,
}

impl<'a, T> std::ops::Deref for ScopeRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for ScopeRef<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&*self.inner, f)
    }
}

pub struct ScopeRefMut<'a, T> {
    inner: RefMut<'a, T>,
}

impl<'a, T> std::ops::Deref for ScopeRefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> std::ops::DerefMut for ScopeRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for ScopeRefMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}
