use nabi;
use abi;
use handle::Handle;

pub struct Stream(Handle);

impl Stream {
    pub fn create() -> nabi::Result<(WriteStream, ReadStream)> {
        let (mut handle_tx, mut handle_rx) = (0, 0);
        let res: nabi::Result<u32> = unsafe {
            abi::stream_create(&mut handle_tx, &mut handle_rx)
        }.into();

        res.map(|_| (WriteStream(Handle(handle_tx)), ReadStream(Handle(handle_rx))))
    }
}

pub struct WriteStream(pub(crate) Handle);

impl WriteStream {
    pub fn send(&self, data: &[u8]) -> nabi::Result<()> {
        let mut written_size = 0;
        let res: nabi::Result<u32> = unsafe {
            abi::stream_write((self.0).0, data.as_ptr(), data.len(), &mut written_size)
        }.into();

        res.map(|_| ())
    }
}

pub struct ReadStream(pub(crate) Handle);

impl ReadStream {
    pub fn read_raw_nonblocking(&self, buffer: &mut [u8]) -> (usize, nabi::Result<()>) {
        let mut msg_size_out = 0;
        let res: nabi::Result<u32> = unsafe {
            abi::stream_read((self.0).0, buffer.as_mut_ptr(), buffer.len(), &mut msg_size_out)
        }.into();

        (msg_size_out, res.map(|_| ()))
    }

    pub fn read_nonblocking(&self) -> nabi::Result<Vec<u8>> {
        let mut faux_buf = [0; 0];
        let (msg_size, _) = self.read_raw_nonblocking(&mut faux_buf);

        let mut buffer = Vec::new();
        buffer.resize(msg_size, 0);
        let (_, res) = self.read_raw_nonblocking(&mut buffer);

        res.map(|_| buffer)
    }

    pub fn read_raw(&self, buffer: &mut [u8]) -> (usize, nabi::Result<()>) {
        let res: nabi::Result<u32> = unsafe {
            abi::object_wait_one((self.0).0, 1 << 0)
        }.into();

        if let Ok(_) = res {
            self.recv_raw_nonblocking(buffer)
        } else {
            (0, res.map(|_| ()))
        }
    }

    pub fn recv(&self) -> nabi::Result<Vec<u8>> {
        let mut faux_buf = [0; 0];
        let (msg_size, _) = self.recv_raw(&mut faux_buf);

        let mut buffer = Vec::new();
        buffer.resize(msg_size, 0);
        let (_, res) = self.recv_raw_nonblocking(&mut buffer);

        res.map(|_| buffer)
    }
}
