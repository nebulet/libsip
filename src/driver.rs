use std::mem;
use std::ops::{Deref, DerefMut};
use super::abi;
use nabi::Result;

pub struct Dma<T>{
    physical_addr: u64,
    sip_addr: *mut T,
}

impl<T> Dma<T> {
    pub unsafe fn new(value: T) -> Result<Dma<T>> {
        let (sip_addr, physical_addr) = physical_alloc::<T>()?;

        sip_addr.write(value);

        Ok(Dma {
            physical_addr,
            sip_addr,
        })
    }

    pub unsafe fn zeroed() -> Result<Dma<T>> {
        // let (sip_addr, physical_addr) = physical_alloc::<T>()?;
        
        // (sip_addr as *mut u8).write_bytes(0, mem::size_of::<T>());

        // Ok(Dma {
        //     physical_addr,
        //     sip_addr,
        // })
        Dma::new(mem::zeroed())

    }

    pub fn physical(&self) -> u64 {
        self.physical_addr
    }
}

impl<T> Deref for Dma<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.sip_addr }
    }
}

impl<T> DerefMut for Dma<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.sip_addr }
    }
}

impl<T> Drop for Dma<T> {
    fn drop(&mut self) {
        unsafe {
            self.sip_addr.drop_in_place();

            // let _ = physical_unmap(self.sip_addr);
        }
    }
}

pub unsafe fn physical_map<T: Sized>(phys_addr: u64) -> Result<*mut T> {
    let page_count = page_count::<T>();

    let res: Result<u32> = abi::physical_map(phys_addr, page_count).into();

    res.map(|offset| offset as _)
}

// pub unsafe fn physical_unmap<T: Sized>(ptr: *mut T) -> Result<()> {
//     let page_count = page_count::<T>();

//     let res: Result<u32> = abi::physical_unmap(ptr as *mut u8, page_count).into();

//     res.map(|_| ())
// }

/// Allocate some physical memory and return the physical address.
pub unsafe fn physical_alloc<T: Sized>() -> Result<(*mut T, u64)> {
    let page_count = page_count::<T>();

    let mut physical_addr = 0;

    let res: Result<u32> = abi::physical_alloc(page_count, &mut physical_addr as *mut _).into();

    res.map(|sip_addr| (sip_addr as *mut T, physical_addr))
}

fn page_count<T: Sized>() -> usize {
    let wasm_page_size = 1 << 16;
    (mem::size_of::<T>() + wasm_page_size - 1) / wasm_page_size
}