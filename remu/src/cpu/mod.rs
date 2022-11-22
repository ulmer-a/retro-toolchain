mod instructions;

const RESET_VECTOR: usize = 0xfffc;
const BRK_VECTOR: usize = 0xfffe;

const IRQ_FLAG: u8 = 1 << 3;
const BRK_FLAG: u8 = 1 << 4;
const ONE_FLAG: u8 = 1 << 5;
const DEC_FLAG: u8 = 1 << 2;

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
    wdc: bool,
}

impl<'a> Cpu<'a> {
    pub fn new(mem: &'a dyn MemoryBus, wdc: bool) -> Self {
        let mut cpu = Self {
            mem,
            x: 0x00,
            y: 0x00,
            a: 0x00,
            p: StatusRegister::new(),
            sp: 0x00,
            pc: 0x0000,
            wdc,
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

    fn is_65c02(&self) -> bool {
        self.wdc
    }

    fn fetch(&mut self) -> u8 {
        let opcode = self.mem.read(self.pc as usize);
        self.pc += 1;
        opcode
    }

    fn push8(&mut self, value: u8) {
        let addr = self.sp as usize + 0x100;
        self.mem.write(addr, value);
        self.sp -= 1;
    }

    fn push16(&mut self, value: u16) {
        let addr = self.sp as usize + 0x100;
        self.mem.write(addr, (value >> 8) as u8);
        self.mem.write(addr - 1, (value & 0xff) as u8);
        self.sp -= 2;
    }
}

struct StatusRegister(u8);

impl StatusRegister {
    fn new() -> Self {
        Self(0)
    }

    fn reset(&mut self) {
        self.0 = ONE_FLAG;
        self.validate();
    }

    fn validate(&self) {
        assert!(self.0 & ONE_FLAG != 0);
        assert!(self.0 & BRK_FLAG == 0);
    }

    fn set(&mut self, flag: u8) {
        self.0 |= flag;
        self.validate();
    }

    fn clr_if(&mut self, flag: u8, cond: bool) {
        if cond {
            self.0 &= !flag;
        }
        self.validate();
    }

    fn as_u8(&self) -> u8 {
        self.validate();
        self.0
    }
}
