impl Z80 {
    
    fn execute_cb_opcode(&mut self) {
        let opcode = self.fetch_byte();
        match opcode {
           
            0x00 => self.rlc_b(),
            
            0x41 => self.bit(0, self.c),
            
            _ => panic!("Unimplemented CB opcode: 0x{:02X}", opcode),
        }
    }

    fn bit(&mut self, bit: u8, reg: u8) {
        self.f.zero = (reg & (1 << bit)) == 0;
        self.f.subtract = false;
        self.f.half_carry = true;
        self.f.parity_overflow = self.f.zero; 
    }
}
