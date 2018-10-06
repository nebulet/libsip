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
// mod mutex;
// mod dlmalloc;
pub mod interrupt;
pub mod driver;

pub use handle::Handle;
pub use wasm::Wasm;
pub use process::Process;
pub use channel::{Channel, WriteChannel, ReadChannel};

pub use event::Event;
// pub use mutex::{Mutex, MutexGuard};

use std::fmt;

pub use nabi::{Result, Error};

// #[global_allocator]
// static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

pub fn print(x: &str) {
    unsafe {
        abi::print(x.as_ptr(), x.len());
    }
}

pub struct PrintWriter;

impl fmt::Write for PrintWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print(s);
        Ok(())
    }
}
