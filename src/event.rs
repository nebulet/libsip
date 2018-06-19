use nabi;
use abi;
use handle::Handle;

pub struct Event(Handle);

impl Event {
    pub fn create() -> nabi::Result<Event> {
        let res: nabi::Result<u32> = unsafe {
            abi::event_create()
        }.into();

        let handle = Handle(res?);

        Ok(Event(handle))
    }

    pub fn wait(&self) -> nabi::Result<()> {
        let res: nabi::Result<u32> = unsafe {
            abi::event_wait((self.0).0)
        }.into();

        res?;

        Ok(())
    }

    pub fn trigger(&self) -> nabi::Result<usize> {
        let res: nabi::Result<u32> = unsafe {
            abi::event_trigger((self.0).0)
        }.into();

        res.map(|count| count as usize)
    }
}
