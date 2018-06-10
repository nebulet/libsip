use nabi;
use abi;
use handle::Handle;

pub struct Wasm(Handle);

impl Wasm {
    pub fn compile(wasm: &[u8]) -> nabi::Result<Handle> {
        let res: nabi::Result<u32> = unsafe {
            abi::wasm_compile(wasm.as_ptr(), wasm.len())
        }.into();

        res.map(|index| Handle(index))
    }
}
