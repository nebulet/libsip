use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut, Drop};
use abi;

pub struct MutexGuard<'a, T: 'a> {
    pfex_item: &'a u8,
    data: &'a mut T,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {} 

pub struct Mutex<T> {
    /// This is forced to start
    /// out as already triggered.
    pfex_item: u8,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Mutex<T> {
        Mutex {
            pfex_item: 0,
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        unsafe {
            abi::pfex_acquire(&(self.pfex_item) as *const u8);
        }

        MutexGuard {
            pfex_item: &self.pfex_item,
            data: unsafe { &mut *self.data.get() },
        }
    }
}

impl<'a, T: 'a> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T: 'a> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T: 'a> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            abi::pfex_release(self.pfex_item as *const u8);
        }
    }
}
