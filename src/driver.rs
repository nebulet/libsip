use super::abi;

pub struct Dma<T>(T);

impl<T> Dma<T> {
    pub fn write(&mut self, value: T) {
        unsafe {
            (&mut self.0 as *mut T).write_volatile(value);
        }
    }

    pub fn read(&self) -> T {
        unsafe {
            (&self.0 as *const T).read_volatile()
        }
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