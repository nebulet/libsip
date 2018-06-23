use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut, Drop};
use abi;

#[repr(transparent)]
struct MutexInner(u32);

pub struct MutexGuard<'a, T: 'a> {
    pfex_item: &'a MutexInner,
    data: &'a mut T,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {} 

pub struct Mutex<T> {
    pfex_item: MutexInner,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Mutex<T> {
        Mutex {
            pfex_item: MutexInner(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        let addr = &self.pfex_item as *const _ as *const u32;
        // println!("lock addr: {:p}", addr);
        unsafe {
            abi::pfex_acquire(addr);
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
            let addr = self.pfex_item as *const _ as *const u32;
            abi::pfex_release(addr);
        }
    }
}
