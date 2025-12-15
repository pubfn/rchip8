use super::Cpu;
use rand;

impl Cpu
{
    pub(super) fn group0(&mut self, opcode: u16)
    {
        match opcode & 0x0FFF
        {
            0x0E0 => todo!(),
            0x0EE =>
            {
                self.reg.pc = self.stack.st[self.stack.sp];
                self.stack.sp -= 1;
            }
            _ => (),
        }
    }

    pub(super) fn x1(&mut self, opcode: usize)
    {
        self.reg.pc = (opcode & 0x0FFF) as u16;
    }

    pub(super) fn x2(&mut self, opcode: usize)
    {
        self.stack.st[self.stack.sp] = self.reg.pc;
        self.stack.sp += 1;
        self.reg.pc = (opcode & 0x0FFF) as u16;
    }

    pub(super) fn x3(&mut self, opcode: usize)
    {
        if self.reg.registers[(opcode & 0x0F00) >> 8] == (opcode & 0x00FF) as u8
        {
            self.inc_pc();
        }
    }

    pub(super) fn x4(&mut self, opcode: usize)
    {
        if !(self.reg.registers[(opcode & 0x0F00) >> 8] == (opcode & 0x00FF) as u8)
        {
            self.inc_pc();
        }
    }

    pub(super) fn x5(&mut self, opcode: usize)
    {
        if self.reg.registers[(opcode & 0x0F00) >> 8] == self.reg.registers[(opcode & 0x00F0) >> 4]
        {
            self.inc_pc();
        }
    }

    pub(super) fn x6(&mut self, opcode: usize)
    {
        self.reg.registers[(opcode & 0x0F00) >> 8] = (opcode & 0x00FF) as u8;
    }

    pub(super) fn x7(&mut self, opcode: usize)
    {
        self.reg.registers[(opcode & 0x0F00) >> 8] += (opcode & 0x00FF) as u8;
    }

    pub(super) fn group8(&mut self, opcode: usize)
    {
        match opcode & 0x000F
        {
            0x0 =>
            {
                self.reg.registers[(opcode & 0x0F00) >> 8] =
                    self.reg.registers[(opcode & 0x00F0) >> 4]
            }
            0x1 =>
            {
                self.reg.registers[(opcode & 0x0F00) >> 8] |=
                    self.reg.registers[(opcode & 0x00F0) >> 4]
            }
            0x2 =>
            {
                self.reg.registers[(opcode & 0x0F00) >> 8] &=
                    self.reg.registers[(opcode & 0x00F0) >> 4]
            }
            0x3 =>
            {
                self.reg.registers[(opcode & 0x0F00) >> 8] ^=
                    self.reg.registers[(opcode & 0x00F0) >> 4]
            }
            0x4 =>
            {
                let (res, overflow) = self.reg.registers[(opcode & 0x0F00) >> 8]
                    .overflowing_add(self.reg.registers[(opcode & 0x00F0) >> 4]);
                self.reg.registers[(opcode & 0x0F00) >> 8] = res;
                self.reg.registers[0xF] = overflow as u8;
            }
            0x5 =>
            {
                let (res, wrap) = self.reg.registers[(opcode & 0x0F00) >> 8]
                    .overflowing_sub(self.reg.registers[(opcode & 0x00F0) >> 4]);
                self.reg.registers[(opcode & 0x0F00) >> 8] = res;
                self.reg.registers[0xF] = wrap as u8;
            }
            0x6 =>
            {
                self.reg.registers[0xF] = self.reg.registers[(opcode & 0x0F00) >> 8] & 0x1;
                self.reg.registers[(opcode & 0x0F00) >> 8] >>= 1;
            }
            0x7 =>
            {
                let (res, wrap) = self.reg.registers[(opcode & 0x00F0) >> 4]
                    .overflowing_sub(self.reg.registers[(opcode & 0x0F00) >> 8]);
                self.reg.registers[(opcode & 0x00F0) >> 4] = res;
                self.reg.registers[0xF] = wrap as u8;
            }
            0xE =>
            {
                self.reg.registers[0xF] = (self.reg.registers[(opcode & 0x0F00) >> 8] & 0x80) >> 7;
                self.reg.registers[(opcode & 0x0F00) >> 8] <<= 1;
            }
            _ => (),
        }
    }

    pub(super) fn x9(&mut self, opcode: usize)
    {
        if !(self.reg.registers[(opcode & 0x0F00) >> 8]
            == self.reg.registers[(opcode & 0x00F0) >> 4])
        {
            self.inc_pc();
        }
    }

    pub(super) fn xA(&mut self, opcode: usize)
    {
        self.reg.i = (opcode & 0x0FFF) as u16
    }

    pub(super) fn xB(&mut self, opcode: usize)
    {
        self.reg.pc = ((opcode & 0x0FFF) as u8 + self.reg.registers[0x0]) as u16;
    }

    pub(super) fn xC(&mut self, opcode: usize)
    {
        let mut rng = rand::random::<u8>();
        rng &= (opcode & 0x00FF) as u8;
        self.reg.registers[(opcode & 0x0F00) >> 8] = rng;
    }

    pub(super) fn xD(&mut self, opcode: usize)
    {
        todo!();
    }

    pub(super) fn groupE(&mut self, opcode: usize)
    {
        match opcode & 0x00FF
        {
            0x9E => todo!(),
            0xA1 => todo!(),
            _ => (),
        }
    }

    pub(super) fn groupF(&mut self, opcode: usize)
    {
        match opcode & 0x00FF
        {
            0x07 => self.reg.registers[(opcode & 0x0F00) >> 8] = self.timers.dt,
            0x0A => todo!(),
            0x15 => self.timers.dt = self.reg.registers[(opcode & 0x0F00) >> 8],
            0x18 => self.timers.st = self.reg.registers[(opcode & 0x0F00) >> 8],
            0x1E => self.reg.i += self.reg.registers[(opcode & 0x0F00) >> 8] as u16,
            0x29 => self.reg.i = (self.reg.registers[(opcode & 0x0F00) >> 8] * 5) as u16,
            0x33 =>
            {
                self.memory[self.reg.i as usize] = self.reg.registers[(opcode & 0x0F00) >> 8] / 100;
                self.memory[(self.reg.i + 1) as usize] =
                    (self.reg.registers[(opcode & 0x0F00) >> 8] / 10) % 10;
                self.memory[(self.reg.i + 2) as usize] =
                    self.reg.registers[(opcode & 0x0F00) >> 8] % 10;
            }
            0x55 =>
            {
                for i in 0..(opcode & 0x0F00) >> 8
                {
                    self.memory[(self.reg.i as usize + i) as usize] = self.reg.registers[i];
                }
            }
            0x65 =>
            {
                for i in 0..(opcode & 0x0F00) >> 8
                {
                    self.reg.registers[i] = self.memory[(self.reg.i as usize + i) as usize];
                }
            }

            _ => (),
        }
    }
}
