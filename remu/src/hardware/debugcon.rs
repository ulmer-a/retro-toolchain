use super::Device;

pub struct DebugConsole {
    base_addr: usize,
}

impl DebugConsole {
    pub fn new(base_addr: usize) -> Self {
        Self { base_addr }
    }
}

impl Device for DebugConsole {
    fn base_addr(&self) -> usize {
        self.base_addr
    }

    fn size(&self) -> usize {
        1
    }

    fn write(&self, _offset: usize, value: u8) {
        print!("{}", value as char);
    }
}
