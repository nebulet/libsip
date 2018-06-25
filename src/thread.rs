use nabi;
use abi;
use std::mem;

const DEFAULT_STACK_SIZE: usize = 1024 * 1024; // 1 MiB

pub struct Thread(u32);

impl Thread {
    pub fn join(self) -> nabi::Result<()> {
        let res: nabi::Result<u32> = unsafe {
            abi::thread_join(self.0)
        }.into();

        res.map(|_| ())
    }
}

pub fn spawn<F>(f: F) -> nabi::Result<Thread> where
    F: FnOnce(), F: Send + 'static
{
    let main = Box::new(f);
    let fptr = Box::into_raw(main);

    let mut stack = vec![0u8; DEFAULT_STACK_SIZE];
    let stack_top = stack.as_mut_ptr() as usize + stack.len();
    mem::forget(stack);
    
    let res: nabi::Result<u32> = unsafe {
        abi::thread_spawn(thread_entry::<F>, fptr as u32, stack_top as *mut u8)
    }.into();

    res.map(|id| Thread(id))
}

pub fn yield_now() {
    unsafe {
        abi::thread_yield()
    }
}

extern fn thread_entry<F>(fptr: u32) where
    F: FnOnce(), F: Send + 'static
{
    let boxed = unsafe { Box::from_raw(fptr as *mut F) };

    boxed();

    // TODO: clean up after the thread
}
