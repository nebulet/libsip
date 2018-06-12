use nabi;
use abi;
use handle::Handle;

pub struct Channel(Handle);

impl Channel {
    pub fn create() -> nabi::Result<(WriteChannel, ReadChannel)> {
        let (mut handle_tx, mut handle_rx) = (0, 0);
        let res: nabi::Result<u32> = unsafe {
            abi::channel_create(&mut handle_tx, &mut handle_rx)
        }.into();

        res.map(|_| (WriteChannel(Handle(handle_tx)), ReadChannel(Handle(handle_rx))))
    }
}

pub struct WriteChannel(Handle);

impl WriteChannel {
    pub fn write(&mut self, data: &[u8]) -> nabi::Result<()> {
        let res: nabi::Result<u32> = unsafe {
            abi::channel_write((self.0).0, data.as_ptr(), data.len())
        }.into();

        res.map(|_| ())
    }
}

pub struct ReadChannel(Handle);

impl ReadChannel {
    pub fn read_raw(&self, buffer: &mut [u8]) -> (usize, nabi::Result<()>) {
        let mut msg_size_out = 0;
        let res: nabi::Result<u32> = unsafe {
            abi::channel_read((self.0).0, buffer.as_mut_ptr(), buffer.len(), &mut msg_size_out)
        }.into();

        (msg_size_out, res.map(|_| ()))
    }

    pub fn read(&self) -> nabi::Result<Vec<u8>> {
        let mut faux_buf = [0; 0];
        let (msg_size, _) = self.read_raw(&mut faux_buf);

        let mut buffer = Vec::with_capacity(msg_size);
        let (_, res) = self.read_raw(&mut buffer);

        res.map(|_| buffer)
    }
}
