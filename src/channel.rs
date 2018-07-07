use nabi;
use abi;
use handle::Handle;

pub struct Channel(Handle);

impl Channel {
    pub const INITIAL: ReadChannel = ReadChannel(Handle(0));
    pub fn create() -> nabi::Result<(WriteChannel, ReadChannel)> {
        let (mut handle_tx, mut handle_rx) = (0, 0);
        let res: nabi::Result<u32> = unsafe {
            abi::channel_create(&mut handle_tx, &mut handle_rx)
        }.into();

        res.map(|_| (WriteChannel(Handle(handle_tx)), ReadChannel(Handle(handle_rx))))
    }
}

pub struct WriteChannel(pub(crate) Handle);

impl WriteChannel {
    pub fn send(&self, data: &[u8]) -> nabi::Result<()> {
        let res: nabi::Result<u32> = unsafe {
            abi::channel_send((self.0).0, data.as_ptr(), data.len())
        }.into();

        res.map(|_| ())
    }
}

pub struct ReadChannel(pub(crate) Handle);

impl ReadChannel {
    pub fn recv_raw_nonblocking(&self, buffer: &mut [u8]) -> (usize, nabi::Result<()>) {
        let mut msg_size_out = 0;
        let res: nabi::Result<u32> = unsafe {
            abi::channel_recv((self.0).0, buffer.as_mut_ptr(), buffer.len(), &mut msg_size_out)
        }.into();

        (msg_size_out, res.map(|_| ()))
    }

    pub fn recv_nonblocking(&self) -> nabi::Result<Vec<u8>> {
        let mut faux_buf = [0; 0];
        let (msg_size, faux_res) = self.recv_raw_nonblocking(&mut faux_buf);

        faux_res?;

        let mut buffer = Vec::new();
        buffer.resize(msg_size, 0);
        let (_, res) = self.recv_raw_nonblocking(&mut buffer);

        res.map(|_| buffer)
    }

    pub fn recv_raw(&self, buffer: &mut [u8]) -> (usize, nabi::Result<()>) {
        let mut faux_buf = [0; 0];
        let (_, faux_res) = self.recv_raw_nonblocking(&mut faux_buf);

        match faux_res {
            Err(nabi::Error::BUFFER_TOO_SMALL) => {
                self.recv_raw_nonblocking(buffer)
            },
            Err(nabi::Error::SHOULD_WAIT) => {
                let res: nabi::Result<u32> = unsafe {
                    abi::object_wait_one((self.0).0, 1 << 0)
                }.into();

                match res {
                    Ok(_) => self.recv_raw_nonblocking(buffer),
                    Err(e) => (0, Err(e)),
                }
            },
            Err(e) => (0, Err(e)),
            Ok(_) => (0, Ok(())),
        }
    }

    pub fn recv(&self) -> nabi::Result<Vec<u8>> {
        let mut faux_buf = [0; 0];
        let (msg_size, faux_res) = self.recv_raw_nonblocking(&mut faux_buf);

        match faux_res {
            Err(nabi::Error::BUFFER_TOO_SMALL) => {
                let mut buffer = Vec::new();
                buffer.resize(msg_size, 0);

                self.recv_raw_nonblocking(&mut buffer).1.map(|_| buffer)
            },
            Err(nabi::Error::SHOULD_WAIT) => {
                let res: nabi::Result<u32> = unsafe {
                    abi::object_wait_one((self.0).0, 1 << 0)
                }.into();

                res?;

                let (msg_size, faux_res) = self.recv_raw_nonblocking(&mut faux_buf);

                match faux_res {
                    Err(nabi::Error::BUFFER_TOO_SMALL) => {
                        let mut buffer = Vec::new();
                        buffer.resize(msg_size, 0);
                        self.recv_raw_nonblocking(&mut buffer).1.map(|_| buffer)
                    },
                    Err(e) => Err(e),
                    Ok(_) => Ok(Vec::new()),
                }
            },
            Err(e) => Err(e),
            Ok(_) => Ok(Vec::new()),
        }
    }
}

impl Iterator for ReadChannel {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Vec<u8>> {
        self.recv().ok()
    }
}