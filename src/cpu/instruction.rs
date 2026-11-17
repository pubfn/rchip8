use super::Cpu;
use crate::hardware::display::Display;
use rand::thread_rng;
use rand::Rng;
impl Cpu
{
    #[inline]
    fn decode(opcode: u16) -> (usize, usize, u8, usize)
    {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as usize;
        (x, y, kk, nnn)
    }

    pub(super) fn group0<D: Display>(&mut self, opcode: u16, display: &mut D)
    {
        match opcode
        {
            0x00E0 =>
            {
                display.clear_display();
                self.inc_pc();
            }
            0x00EE =>
            {
                self.stack.sp -= 1;
                self.reg.pc = self.stack.st[self.stack.sp];
            }
            _ =>
            {}
        }
    }

    pub(super) fn x1(&mut self, opcode: u16)
    {
        let (_, _, _, nnn) = Self::decode(opcode);
        self.reg.pc = nnn as u16;
    }

    pub(super) fn x2(&mut self, opcode: u16)
    {
        let (_, _, _, nnn) = Self::decode(opcode);
        self.stack.st[self.stack.sp] = self.reg.pc + 2;
        self.stack.sp += 1;
        self.reg.pc = nnn as u16;
    }

    pub(super) fn x3(&mut self, opcode: u16)
    {
        let (x, _, kk, _) = Self::decode(opcode);
        if self.reg.registers[x] == kk
        {
            self.inc_pc();
        }
    }

    pub(super) fn x4(&mut self, opcode: u16)
    {
        let (x, _, kk, _) = Self::decode(opcode);
        if self.reg.registers[x] != kk
        {
            self.inc_pc();
        }
    }

    pub(super) fn x5(&mut self, opcode: u16)
    {
        let (x, y, _, _) = Self::decode(opcode);
        if self.reg.registers[x] == self.reg.registers[y]
        {
            self.inc_pc();
        }
    }

    pub(super) fn x6(&mut self, opcode: u16)
    {
        let (x, _, kk, _) = Self::decode(opcode);
        self.reg.registers[x] = kk;
    }

    pub(super) fn x7(&mut self, opcode: u16)
    {
        let (x, _, kk, _) = Self::decode(opcode);
        self.reg.registers[x] = self.reg.registers[x].wrapping_add(kk);
    }

    pub(super) fn group8(&mut self, opcode: u16)
    {
        let (x, y, _, _) = Self::decode(opcode);

        match opcode & 0x000F
        {
            0x0 => self.reg.registers[x] = self.reg.registers[y],

            0x1 => self.reg.registers[x] |= self.reg.registers[y],

            0x2 => self.reg.registers[x] &= self.reg.registers[y],

            0x3 => self.reg.registers[x] ^= self.reg.registers[y],

            0x4 =>
            {
                let (res, carry) = self.reg.registers[x].overflowing_add(self.reg.registers[y]);
                self.reg.registers[x] = res;
                self.reg.registers[0xF] = carry as u8;
            }

            0x5 =>
            {
                let (res, borrow) = self.reg.registers[x].overflowing_sub(self.reg.registers[y]);
                self.reg.registers[x] = res;
                self.reg.registers[0xF] = (!borrow) as u8;
            }

            0x6 =>
            {
                let vy = self.reg.registers[y];

                self.reg.registers[x] = vy >> 1;
                self.reg.registers[0xF] = vy & 0x1;
            }

            0x7 =>
            {
                let (res, borrow) = self.reg.registers[y].overflowing_sub(self.reg.registers[x]);
                self.reg.registers[x] = res;
                self.reg.registers[0xF] = (!borrow) as u8;
            }
            0xE =>
            {
                let vy = self.reg.registers[y];

                self.reg.registers[x] = vy << 1;
                self.reg.registers[0xF] = (vy & 0x80) >> 7;
            }

            _ =>
            {}
        }
    }

    pub(super) fn x9(&mut self, opcode: u16)
    {
        let (x, y, _, _) = Self::decode(opcode);
        if self.reg.registers[x] != self.reg.registers[y]
        {
            self.inc_pc();
        }
    }

    pub(super) fn xA(&mut self, opcode: u16)
    {
        let (_, _, _, nnn) = Self::decode(opcode);
        self.reg.i = nnn as u16;
    }

    pub(super) fn xB(&mut self, opcode: u16)
    {
        let (_, _, _, nnn) = Self::decode(opcode);
        self.reg.pc = (nnn + self.reg.registers[0] as usize) as u16;
    }

    pub(super) fn xC(&mut self, opcode: u16)
    {
        let (x, _, kk, _) = Self::decode(opcode);
        let mut rng = thread_rng();
        let val: u8 = rng.r#gen();
        self.reg.registers[x] = val & kk;
    }

    pub(super) fn xD<D: Display>(&mut self, opcode: u16, display: &mut D)
    {
        let (x, y, _, _) = Self::decode(opcode);
        let height = (opcode & 0x000F) as usize;

        self.reg.registers[0xF] = 0;

        let vx = self.reg.registers[x] as usize;
        let vy = self.reg.registers[y] as usize;

        for row in 0..height
        {
            let sprite = self.memory[self.reg.i as usize + row];
            let y_coord = (vy + row) % 32;

            for col in 0..8
            {
                let pixel = ((sprite >> (7 - col)) & 1) == 1;
                let x_coord = (vx + col) % 64;

                let old = display.get_pixel(x_coord, y_coord);

                if old && pixel
                {
                    self.reg.registers[0xF] = 1;
                }

                display.set_pixel(x_coord, y_coord, old ^ pixel);
            }
        }
    }

    pub(super) fn groupE(&mut self, opcode: u16, key: Option<usize>)
    {
        let (x, _, _, _) = Self::decode(opcode);
        let vx = (self.reg.registers[x] & 0xF) as usize;

        match opcode & 0x00FF
        {
            0x9E =>
            {
                if key == Some(vx)
                {
                    self.inc_pc();
                }
            }
            0xA1 =>
            {
                if key != Some(vx)
                {
                    self.inc_pc();
                }
            }
            _ =>
            {}
        }
    }

    pub(super) fn groupF(&mut self, opcode: u16, key: Option<usize>)
    {
        let (x, _, _, _) = Self::decode(opcode);

        match opcode & 0x00FF
        {
            0x07 => self.reg.registers[x] = self.timers.dt,

            0x0A =>
            {
                if let Some(k) = key
                {
                    self.reg.registers[x] = k as u8;
                }
                else
                {
                    self.reg.pc -= 2;
                }
            }

            0x15 => self.timers.dt = self.reg.registers[x],
            0x18 => self.timers.st = self.reg.registers[x],

            0x1E =>
            {
                self.reg.i = self.reg.i.wrapping_add(self.reg.registers[x] as u16);
            }

            0x29 =>
            {
                self.reg.i = (self.reg.registers[x] as u16) * 5;
            }

            0x33 =>
            {
                let i = self.reg.i as usize;

                let vx = self.reg.registers[x];
                self.memory[i] = vx / 100;
                self.memory[i + 1] = (vx / 10) % 10;
                self.memory[i + 2] = vx % 10;
            }

            0x55 =>
            {
                let i = self.reg.i as usize;

                for idx in 0..=x
                {
                    self.memory[i + idx] = self.reg.registers[idx];
                }
            }

            0x65 =>
            {
                let i = self.reg.i as usize;
                if i + x >= self.memory.len()
                {
                    return;
                }

                for idx in 0..=x
                {
                    self.reg.registers[idx] = self.memory[i + idx];
                }
            }

            _ =>
            {}
        }
    }
}
