use {Event, Handle};
use nabi;
use abi;

pub unsafe fn create_irq_event(irq: u8) -> nabi::Result<Event> {
    let res: nabi::Result<u32> = {
        abi::create_irq_event(irq)
    }.into();

    let handle = Handle(res?);

    Ok(Event(handle))
}
