mod debugcon;
pub use debugcon::DebugConsole;

mod memory;
pub use memory::MemoryDevice;

pub trait Device {
    fn base_addr(&self) -> usize;
    fn size(&self) -> usize;

    fn responds_to(&self, addr: usize) -> bool {
        addr >= self.base_addr() && addr < (self.base_addr() + self.size())
    }

    fn read(&self, _offset: usize) -> u8 {
        0
    }

    fn write(&self, _offset: usize, _value: u8) {}
}
