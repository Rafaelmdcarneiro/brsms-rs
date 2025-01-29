impl Z80 {
    fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.memory.read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        word
    }

    // NOP instruction
    fn nop(&mut self) {
        // No operation
    }

    // LD BC, nn
    fn ld_bc_nn(&mut self) {
        let nn = self.fetch_word();
        self.set_bc(nn);
    }

    // ADD A, B
    fn add_a_b(&mut self) {
        let a = self.a;
        let b = self.b;
        let result = a.wrapping_add(b);
        self.a = result;

        self.f.zero = result == 0;
        self.f.sign = (result & 0x80) != 0;
        self.f.carry = (a as u16 + b as u16) > 0xFF;
        self.f.half_carry = (a & 0x0F) + (b & 0x0F) > 0x0F;
        self.f.parity_overflow = ((a ^ b) & 0x80) == 0 && ((a ^ result) & 0x80) != 0;
        self.f.subtract = false;
    }

    // Execute next instruction
    fn execute_next(&mut self) {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => self.nop(),
            0x01 => self.ld_bc_nn(),
            0x80 => self.add_a_b(),
            _ => panic!("Unimplemented opcode: 0x{:02X}", opcode),
        }
    }
}
