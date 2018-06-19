use nabi;
use abi;
use handle::Handle;
use channel::ReadChannel;
use wasm::Wasm;

pub struct Process(Handle);

impl Process {
    pub fn create(wasm: Wasm, channel: ReadChannel) -> nabi::Result<Process> {
        use std::mem;
        let res: nabi::Result<u32> = unsafe {
            abi::process_create((wasm.0).0, (channel.0).0)
        }.into();

        mem::forget(channel);

        res.map(|index| Process(Handle(index)))
    }

    pub fn start(&self) -> nabi::Result<()> {
        let res: nabi::Result<u32> = unsafe {
            abi::process_start((self.0).0)
        }.into();

        res.map(|_| ())
    }
}
