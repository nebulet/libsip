#![feature(
    const_fn,
    link_llvm_intrinsics,
    allocator_api,
    naked_functions,
)]

extern crate nabi;

#[macro_use]
mod print;
pub mod abi;
mod types;
mod handle;
mod wasm;
mod process;
mod channel;
mod event;
mod mutex;
pub mod thread;
mod dlmalloc;
pub mod interrupt;

pub use handle::Handle;
pub use wasm::Wasm;
pub use process::Process;
pub use channel::{Channel, WriteChannel, ReadChannel};

pub use event::Event;
pub use mutex::{Mutex, MutexGuard};

use std::fmt;

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

pub fn print(x: &str) {
    unsafe {
        abi::print(x.as_ptr(), x.len());
    }
}

pub fn physical_map<T: Sized>(phys_addr: u64) -> nabi::Result<&'static mut T> {
    use std::mem;

    let page_count = {
        let rem = mem::size_of::<T>() % (1 << 16);
        mem::size_of::<T>() + (1 << 16) - rem
    };

    let res: nabi::Result<u32> = unsafe {
        abi::physical_map(phys_addr, page_count)
    }.into();

    res.map(|offset| unsafe { mem::transmute(offset) })
}

pub struct PrintWriter;

impl fmt::Write for PrintWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print(s);
        Ok(())
    }
}
