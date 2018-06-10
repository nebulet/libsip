use std::convert::From;
use nabi;

#[repr(C)]
pub struct AbiResult(u64);

impl From<AbiResult> for nabi::Result<u32> {
    fn from(res: AbiResult) -> nabi::Result<u32> {
        nabi::Error::demux(res.0)
    }
}
