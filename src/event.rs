use nabi;
use abi;
use handle::Handle;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EventState {
    Pending = 0,
    Done = 1,
}

pub struct Event(Handle);

impl Event {
    pub fn create() -> nabi::Result<Event> {
        let res: nabi::Result<u32> = unsafe {
            abi::event_create()
        }.into();

        let handle = Handle(res?);

        Ok(Event(handle))
    }

    pub fn poll(&self) -> nabi::Result<EventState> {
        let res: nabi::Result<u32> = unsafe {
            abi::event_poll((self.0).0)
        }.into();

        res.map(|s| match s {
            0 => EventState::Pending,
            1 => EventState::Done,
            _ => unimplemented!(),
        })
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

    pub fn rearm(&self) -> nabi::Result<()> {
        let res: nabi::Result<u32> = unsafe {
            abi::event_rearm((self.0).0)
        }.into();

        res.map(|_| ())
    }
}
