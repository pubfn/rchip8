use super::Cpu;

impl Cpu
{
    pub(super) fn Group0(&mut self, opcode: u16)
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

    pub(super) fn jump(&mut self, opcode: u16)
    {
        self.reg.pc = opcode & 0x0FFF;
    }

    pub(super) fn call(&mut self, opcode: u16)
    {
        self.stack.st[self.stack.sp] = self.reg.pc;
        self.stack.sp += 1;
        self.reg.pc = opcode & 0x0FFF;
    }

    pub(super) fn skip(&mut self, opcode: usize)
    {
        if self.reg.registers[(opcode & 0x0F00) >> 8] == (opcode & 0x00FF) as u8
        {
            self.inc_pc();
        }
    }
}
