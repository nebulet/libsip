use nabi;
use abi;
use handle::Handle;

pub struct Process(Handle);

impl Process {
    pub fn create(wasm: Handle, channel: Handle) -> nabi::Result<Handle> {
        use std::mem;
        let res: nabi::Result<u32> = unsafe {
            abi::process_create(wasm.0, channel.0)
        }.into();

        mem::forget(channel);

        res.map(|index| Handle(index))
    }

    pub fn start(&self) -> nabi::Result<()> {
        let res: nabi::Result<u32> = unsafe {
            abi::process_start((self.0).0)
        }.into();

        res.map(|_| ())
    }
}
