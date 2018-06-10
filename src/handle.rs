use abi;
use nabi;

pub struct Handle(pub(crate) u32);

impl Handle {
    pub fn duplicate(&self, new_rights: nabi::HandleRights) -> nabi::Result<Handle> {
        let res: nabi::Result<u32> = unsafe {
            abi::handle_duplicate(self.0, new_rights.bits())
        }.into();

        res.map(|index| Handle(index))
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            abi::handle_close(self.0);
        }
    }
}
