use handle::Handle;
use nabi;
use abi;

pub struct Thread(Handle);

pub fn spawn<F>(f: F) -> nabi::Result<Thread> where
    F: FnOnce(), F: Send + 'static
{
    let main = Box::new(f);
    let fptr = Box::into_raw(main);
    
    let res: nabi::Result<u32> = unsafe {
        abi::thread_spawn(thread_entry::<F>, fptr as u32)
    }.into();

    res.map(|handle| Thread(Handle(handle)))
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
}
