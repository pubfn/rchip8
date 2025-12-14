mod instruction;

struct Stack
{
    st: [u16; 16],
    sp: usize,
}

struct Registers
{
    registers: [u8; 16],
    i: u16,
    pc: u16,
}

struct Timers
{
    dt: u8, //delay timer
    st: u8, //soundtimer
}

pub struct Cpu
{
    timers: Timers,
    stack: Stack,
    reg: Registers,
    memory: [u8; 4096],
}

impl Cpu
{
    pub fn new() -> Self
    {
        Cpu {
            stack: Stack {
                st: [0u16; 16],
                sp: 0,
            },
            timers: Timers { dt: 0, st: 0 },
            reg: Registers {
                registers: [0u8; 16],
                i: 0,
                pc: 0,
            },
            memory: [0u8; 4096],
        }
    }

    pub fn inc_pc(&mut self)
    {
        self.reg.pc += 2;
    }
}
