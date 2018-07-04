use nabi;
use abi;
use handle::Handle;

pub struct Event(pub(crate) Handle);

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
            abi::object_wait_one((self.0).0, 1 << 4)
        }.into();

        res?;

        Ok(())
    }

    pub fn signal(&self) -> nabi::Result<usize> {
        let res: nabi::Result<u32> = unsafe {
            abi::object_signal((self.0).0, 1 << 4, 0)
        }.into();

        res.map(|count| count as usize)
    }

    pub fn rearm(&self) -> nabi::Result<()> {
        let res: nabi::Result<u32> = unsafe {
            abi::object_signal((self.0).0, 0, 1 << 4)
        }.into();

        res.map(|_| ())
    }
}
