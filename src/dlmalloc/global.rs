use std::alloc::{Alloc, Layout, GlobalAlloc, AllocErr};
use std::ptr::NonNull;

use super::super::{Mutex, MutexGuard};
use super::Dlmalloc;

pub struct GlobalDlmalloc;

unsafe impl GlobalAlloc for GlobalDlmalloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        <Dlmalloc>::malloc(&mut get(), layout.size(), layout.align()) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        <Dlmalloc>::free(&mut get(), ptr as *mut u8, layout.size(), layout.align())
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        <Dlmalloc>::calloc(&mut get(), layout.size(), layout.align()) as *mut u8
    }

    #[inline]
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize
    ) -> *mut u8 {
        <Dlmalloc>::realloc(
            &mut get(),
            ptr as *mut u8,
            layout.size(),
            layout.align(),
            new_size,
        ) as *mut u8
    }
}

unsafe impl Alloc for GlobalDlmalloc {
    #[inline]
    unsafe fn alloc(
        &mut self,
        layout: Layout
    ) -> Result<NonNull<u8>, AllocErr> {
        get().alloc(layout)
    }

    #[inline]
    unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        get().dealloc(ptr, layout)
    }

    #[inline]
    unsafe fn realloc(
        &mut self,
        ptr: NonNull<u8>,
        layout: Layout,
        new_size: usize
    ) -> Result<NonNull<u8>, AllocErr> {
        Alloc::realloc(&mut *get(), ptr, layout, new_size)
    }

    #[inline]
    unsafe fn alloc_zeroed(
        &mut self,
        layout: Layout
    ) -> Result<NonNull<u8>, AllocErr> {
        get().alloc_zeroed(layout)
    }
}

static DLMALLOC: Mutex<Dlmalloc> = Mutex::new(Dlmalloc(super::dlmalloc::DLMALLOC_INIT));

fn get() -> MutexGuard<'static, Dlmalloc> {
    DLMALLOC.lock()
}
