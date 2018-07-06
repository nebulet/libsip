use nabi;
use abi;
use handle::Handle;
use channel::WriteChannel;

pub struct Interrupt(Handle);

impl Interrupt {
    pub fn create(channel: WriteChannel, vector: u8) -> nabi::Result<Interrupt> {
        let res: nabi::Result<u32> = unsafe {
            abi::interrupt_create((channel.0).0, vector)
        }.into();

        res.map(|handle| Interrupt(Handle(handle)))
    }

    pub fn ack(&self) -> nabi::Result<()> {
        let res: nabi::Result<u32> = unsafe {
            abi::interrupt_ack((self.0).0)
        }.into();

        res.map(|_| ())
    }
}