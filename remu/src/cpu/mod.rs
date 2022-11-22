mod instructions;

const RESET_VECTOR: usize = 0xfffc;

pub trait MemoryBus {
    fn read(&self, addr: usize) -> u8;

    fn read16(&self, addr: usize) -> u16 {
        (self.read(addr) as u16) | ((self.read(addr + 1) as u16) << 8)
    }

    fn write(&self, addr: usize, value: u8);
}

pub struct Cpu<'a> {
    mem: &'a dyn MemoryBus,
    x: u8,
    y: u8,
    a: u8,
    p: StatusRegister,
    sp: u8,
    pc: u16,
}

struct StatusRegister {}

impl<'a> Cpu<'a> {
    pub fn new(mem: &'a dyn MemoryBus) -> Self {
        let mut cpu = Self {
            mem,
            x: 0x00,
            y: 0x00,
            a: 0x00,
            p: StatusRegister::new(),
            sp: 0x00,
            pc: 0x0000,
        };
        cpu.reset();
        cpu
    }

    pub fn reset(&mut self) {
        self.pc = self.mem.read16(RESET_VECTOR);
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xfd;
        self.p.reset();
    }

    pub fn step(&mut self) {
        use instructions::OPCODE_TABLE;
        let opcode = self.fetch();
        OPCODE_TABLE[opcode as usize](self);
    }

    fn fetch(&mut self) -> u8 {
        let opcode = self.mem.read(self.pc as usize);
        self.pc += 1;
        opcode
    }
}

impl StatusRegister {
    fn new() -> Self {
        Self {}
    }

    fn reset(&mut self) {}
}
