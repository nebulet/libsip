#![feature(
    wasm_import_module,
)]

extern crate nabi;

pub mod abi;
mod types;
mod handle;
mod wasm;
mod process;
mod channel;

pub use handle::Handle;
pub use wasm::Wasm;
pub use process::Process;
pub use channel::{Channel, WriteChannel, ReadChannel};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::print(&format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

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
