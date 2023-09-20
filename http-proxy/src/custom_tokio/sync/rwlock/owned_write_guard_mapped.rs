use crate::custom_tokio::sync::rwlock::RwLock;
use std::marker::PhantomData;
use std::sync::Arc;
use std::{fmt, mem, ops, ptr};

/// Owned RAII structure used to release the exclusive write access of a lock when
/// dropped.
///
/// This structure is created by [mapping] an [`OwnedRwLockWriteGuard`]. It is a
/// separate type from `OwnedRwLockWriteGuard` to disallow downgrading a mapped
/// guard, since doing so can cause undefined behavior.
///
/// [mapping]: method@crate::custom_tokio::sync::OwnedRwLockWriteGuard::map
/// [`OwnedRwLockWriteGuard`]: struct@crate::custom_tokio::sync::OwnedRwLockWriteGuard
#[clippy::has_significant_drop]
pub(crate) struct OwnedRwLockMappedWriteGuard<T: ?Sized, U: ?Sized = T> {
    // When changing the fields in this struct, make sure to update the
    // `skip_drop` method.
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    pub(crate) resource_span: tracing::Span,
    pub(crate) permits_acquired: u32,
    pub(crate) lock: Arc<RwLock<T>>,
    pub(crate) data: *mut U,
    pub(crate) _p: PhantomData<T>,
}

#[allow(dead_code)] // Unused fields are still used in Drop.
struct Inner<T: ?Sized, U: ?Sized> {
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    resource_span: tracing::Span,
    permits_acquired: u32,
    lock: Arc<RwLock<T>>,
    data: *const U,
}

impl<T: ?Sized, U: ?Sized> OwnedRwLockMappedWriteGuard<T, U> {
    fn skip_drop(self) -> Inner<T, U> {
        let me = mem::ManuallyDrop::new(self);
        // SAFETY: This duplicates the values in every field of the guard, then
        // forgets the originals, so in the end no value is duplicated.
        unsafe {
            Inner {
                #[cfg(all(tokio_unstable, feature = "tracing"))]
                resource_span: ptr::read(&me.resource_span),
                permits_acquired: me.permits_acquired,
                lock: ptr::read(&me.lock),
                data: me.data,
            }
        }
    }

    /// Makes a new `OwnedRwLockMappedWriteGuard` for a component of the locked
    /// data.
    ///
    /// This operation cannot fail as the `OwnedRwLockMappedWriteGuard` passed
    /// in already locked the data.
    ///
    /// This is an associated function that needs to be used as
    /// `OwnedRwLockWriteGuard::map(..)`. A method would interfere with methods
    /// of the same name on the contents of the locked data.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use crate::custom_tokio::sync::{RwLock, OwnedRwLockWriteGuard};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// struct Foo(u32);
    ///
    /// # #[crate::custom_tokio::main]
    /// # async fn main() {
    /// let lock = Arc::new(RwLock::new(Foo(1)));
    ///
    /// {
    ///     let lock = Arc::clone(&lock);
    ///     let mut mapped = OwnedRwLockWriteGuard::map(lock.write_owned().await, |f| &mut f.0);
    ///     *mapped = 2;
    /// }
    ///
    /// assert_eq!(Foo(2), *lock.read().await);
    /// # }
    /// ```
    #[inline]
    pub(crate) fn map<F, V: ?Sized>(mut this: Self, f: F) -> OwnedRwLockMappedWriteGuard<T, V>
    where
        F: FnOnce(&mut U) -> &mut V,
    {
        let data = f(&mut *this) as *mut V;
        let this = this.skip_drop();

        OwnedRwLockMappedWriteGuard {
            permits_acquired: this.permits_acquired,
            lock: this.lock,
            data,
            _p: PhantomData,
            #[cfg(all(tokio_unstable, feature = "tracing"))]
            resource_span: this.resource_span,
        }
    }

    /// Attempts to make a new `OwnedRwLockMappedWriteGuard` for a component
    /// of the locked data. The original guard is returned if the closure
    /// returns `None`.
    ///
    /// This operation cannot fail as the `OwnedRwLockMappedWriteGuard` passed
    /// in already locked the data.
    ///
    /// This is an associated function that needs to be
    /// used as `OwnedRwLockMappedWriteGuard::try_map(...)`. A method would interfere with
    /// methods of the same name on the contents of the locked data.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use crate::custom_tokio::sync::{RwLock, OwnedRwLockWriteGuard};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// struct Foo(u32);
    ///
    /// # #[crate::custom_tokio::main]
    /// # async fn main() {
    /// let lock = Arc::new(RwLock::new(Foo(1)));
    ///
    /// {
    ///     let guard = Arc::clone(&lock).write_owned().await;
    ///     let mut guard = OwnedRwLockWriteGuard::try_map(guard, |f| Some(&mut f.0)).expect("should not fail");
    ///     *guard = 2;
    /// }
    ///
    /// assert_eq!(Foo(2), *lock.read().await);
    /// # }
    /// ```
    #[inline]
    pub(crate) fn try_map<F, V: ?Sized>(
        mut this: Self,
        f: F,
    ) -> Result<OwnedRwLockMappedWriteGuard<T, V>, Self>
    where
        F: FnOnce(&mut U) -> Option<&mut V>,
    {
        let data = match f(&mut *this) {
            Some(data) => data as *mut V,
            None => return Err(this),
        };
        let this = this.skip_drop();

        Ok(OwnedRwLockMappedWriteGuard {
            permits_acquired: this.permits_acquired,
            lock: this.lock,
            data,
            _p: PhantomData,
            #[cfg(all(tokio_unstable, feature = "tracing"))]
            resource_span: this.resource_span,
        })
    }
}

impl<T: ?Sized, U: ?Sized> ops::Deref for OwnedRwLockMappedWriteGuard<T, U> {
    type Target = U;

    fn deref(&self) -> &U {
        unsafe { &*self.data }
    }
}

impl<T: ?Sized, U: ?Sized> ops::DerefMut for OwnedRwLockMappedWriteGuard<T, U> {
    fn deref_mut(&mut self) -> &mut U {
        unsafe { &mut *self.data }
    }
}

impl<T: ?Sized, U: ?Sized> fmt::Debug for OwnedRwLockMappedWriteGuard<T, U>
where
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized, U: ?Sized> fmt::Display for OwnedRwLockMappedWriteGuard<T, U>
where
    U: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized, U: ?Sized> Drop for OwnedRwLockMappedWriteGuard<T, U> {
    fn drop(&mut self) {
        self.lock.s.release(self.permits_acquired as usize);

        #[cfg(all(tokio_unstable, feature = "tracing"))]
        self.resource_span.in_scope(|| {
            tracing::trace!(
            target: "runtime::resource::state_update",
            write_locked = false,
            write_locked.op = "override",
            )
        });
    }
}
