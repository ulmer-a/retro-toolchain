use super::Device;
use std::{
    cell::UnsafeCell,
    fs::File,
    io::{self, Read},
};

pub struct MemoryDevice {
    base_addr: usize,
    size: usize,
    readonly: bool,
    data: UnsafeCell<Vec<u8>>,
}

impl MemoryDevice {
    pub fn new(base_addr: usize, size: usize, readonly: bool) -> Self {
        Self {
            base_addr,
            size,
            readonly,
            data: UnsafeCell::new(vec![0; size]),
        }
    }

    pub fn load(&self, file: &str) -> io::Result<()> {
        let mut f = File::open(file)?;
        f.read_exact(unsafe { &mut *self.data.get().as_mut().unwrap() })
    }

    pub fn ram(base_addr: usize, size: usize) -> Self {
        Self::new(base_addr, size, false)
    }

    pub fn rom(base_addr: usize, size: usize, file: &str) -> io::Result<Self> {
        let dev = Self::new(base_addr, size, true);
        dev.load(file)?;
        Ok(dev)
    }
}

impl Device for MemoryDevice {
    fn base_addr(&self) -> usize {
        self.base_addr
    }

    fn size(&self) -> usize {
        self.size
    }

    fn read(&self, offset: usize) -> u8 {
        unsafe { (&self.data.get().as_ref().unwrap())[offset] }
    }

    fn write(&self, offset: usize, value: u8) {
        if self.readonly {
            return;
        }

        unsafe {
            (&mut self.data.get().as_mut().unwrap())[offset] = value;
        }
    }
}
