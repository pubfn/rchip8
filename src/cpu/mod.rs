mod instruction;
use raylib::RaylibHandle;

use crate::hardware::{display::Display, keyboard::Keyboard};

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
                pc: 0x200,
            },
            memory: [0u8; 4096],
        }
    }

    pub fn load_to_memory(&mut self, rom: &[u8], font: &[u8])
    {
        self.memory[0x50..0x50 + font.len()].copy_from_slice(font);
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(rom);
    }

    pub fn get_sound_timer(&self) -> u8
    {
        self.timers.st
    }

    pub fn timers_step(&mut self)
    {
        self.timers.dt = self.timers.dt.saturating_sub(1);
        self.timers.st = self.timers.st.saturating_sub(1);
    }

    pub fn execute(&mut self, display: &mut Display, rl: &RaylibHandle)
    {
        let byte1 = self.memory[self.reg.pc as usize];
        let byte2 = self.memory[(self.reg.pc + 1) as usize];

        let opcode = ((byte1 as u16) << 8) | (byte2 as u16);

        match opcode >> 12
        {
            0x0 => self.group0(opcode, display),

            0x1 => self.x1(opcode),

            0x2 => self.x2(opcode),

            0x3 =>
            {
                self.x3(opcode);
                self.inc_pc();
            }

            0x4 =>
            {
                self.x4(opcode);
                self.inc_pc();
            }

            0x5 =>
            {
                self.x5(opcode);
                self.inc_pc();
            }

            0x6 =>
            {
                self.x6(opcode);
                self.inc_pc();
            }

            0x7 =>
            {
                self.x7(opcode);
                self.inc_pc();
            }

            0x8 =>
            {
                self.group8(opcode);
                self.inc_pc();
            }

            0x9 =>
            {
                self.x9(opcode);
                self.inc_pc();
            }

            0xA =>
            {
                self.xA(opcode);
                self.inc_pc();
            }

            0xB => self.xB(opcode),

            0xC =>
            {
                self.xC(opcode);
                self.inc_pc();
            }

            0xD =>
            {
                self.xD(opcode, display);
                self.inc_pc();
            }

            0xE =>
            {
                let key = Keyboard::get_key(&rl);
                self.groupE(opcode, key);
                self.inc_pc();
            }

            0xF =>
            {
                let key = Keyboard::get_key_pressed(&rl);
                self.groupF(opcode, key);
                self.inc_pc();
            }

            _ =>
            {}
        }
    }

    pub fn inc_pc(&mut self)
    {
        self.reg.pc += 2;
    }
}
