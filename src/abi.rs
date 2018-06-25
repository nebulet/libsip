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

    // handles
    pub fn handle_close(handle: u32) -> AbiResult;
    pub fn handle_duplicate(handle: u32, new_rights: u32) -> AbiResult;

    // drivers
    pub fn physical_map(phys_addr: u64, page_count: usize) -> AbiResult;
    pub fn read_port_u8(port: u16) -> u8;
    pub fn write_port_u8(port: u16, val: u8);
    pub fn set_irq_handler(index: u8, handler: unsafe extern fn());

    // events
    pub fn event_create() -> AbiResult;
    pub fn event_wait(handle: u32) -> AbiResult;
    pub fn event_poll(handle: u32) -> AbiResult;
    pub fn event_trigger(handle: u32) -> AbiResult;
    pub fn event_rearm(handle: u32) -> AbiResult;

    // Pretty fast Exclusion
    pub fn pfex_acquire(state_ptr: *const u32);
    pub fn pfex_release(state_ptr: *const u32);

    // threads
    pub fn thread_yield();
    pub fn thread_spawn(f: extern fn(u32), arg: u32, stack_ptr: *mut u8) -> AbiResult;
    pub fn thread_join(id: u32) -> AbiResult;
}
