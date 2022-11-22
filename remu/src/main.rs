mod cpu;
use cpu::{Cpu, MemoryBus};

use hardware::Device;
mod hardware;

fn main() {
    println!("Hello, world!");

    let system = SbcSystem::new()
        .add_device(Box::new(hardware::DebugConsole::new(0xdfd0)))
        .add_device(Box::new(hardware::MemoryDevice::ram(0x0000, 8192)))
        .add_device(Box::new(
            hardware::MemoryDevice::rom(0x0000, 8192, "test.txt").unwrap(),
        ));

    let mut cpu = Cpu::new(&system);

    loop {
        cpu.step();
    }
}

pub struct SbcSystem {
    devices: Vec<Box<dyn Device>>,
}

impl SbcSystem {
    fn new() -> Self {
        Self { devices: vec![] }
    }

    fn add_device(mut self, device: Box<dyn Device>) -> Self {
        self.devices.push(device);
        self
    }
}

impl MemoryBus for SbcSystem {
    fn read(&self, addr: usize) -> u8 {
        let mut value = 0;
        for device in &self.devices {
            if device.responds_to(addr) {
                value |= device.read(addr);
            }
        }
        value
    }

    fn write(&self, addr: usize, value: u8) {
        for device in &self.devices {
            if device.responds_to(addr) {
                device.write(addr, value);
            }
        }
    }
}
