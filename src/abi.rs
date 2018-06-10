use types::AbiResult;

#[wasm_import_module = "abi"]
extern {
    pub fn print(ptr: *const u8, len: usize);
    pub fn wasm_compile(ptr: *const u8, len: usize) -> AbiResult;
    pub fn process_create(code_handle: u32, chan_handle: u32) -> AbiResult;
    pub fn process_start(process_handle: u32) -> AbiResult;
    
    pub fn channel_create(handle0: &mut u32, handle1: &mut u32) -> AbiResult;
    pub fn channel_write(handle: u32, ptr: *const u8, len: usize) -> AbiResult;
    pub fn channel_read(handle: u32, ptr: *mut u8, len: usize, msg_len_out: &mut usize) -> AbiResult;

    pub fn physical_map(phys_addr: u64, page_count: usize) -> AbiResult;

    // handles
    pub fn handle_close(handle: u32) -> AbiResult;
    pub fn handle_duplicate(handle: u32, new_rights: u32) -> AbiResult;
}
