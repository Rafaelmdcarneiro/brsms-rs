impl Z80 {
    // ADD A, n
    fn add_a_n(&mut self) {
        let n = self.fetch_byte();
        let a = self.a;
        let result = a.wrapping_add(n);
        self.a = result;
        self.update_flags_add(a, n, result);
    }

    // SUB A, n
    fn sub_a_n(&mut self) {
        let n = self.fetch_byte();
        let a = self.a;
        let result = a.wrapping_sub(n);
        self.a = result;
        self.update_flags_sub(a, n, result);
    }

    // ADC HL, BC
    fn adc_hl_bc(&mut self) {
        let hl = self.get_hl();
        let bc = self.get_bc();
        let carry = self.f.carry as u16;
        let result = hl.wrapping_add(bc).wrapping_add(carry);
        self.set_hl(result);
        self.update_flags_adc16(hl, bc, result);
    }

    fn update_flags_add(&mut self, a: u8, b: u8, result: u8) {
        self.f.zero = result == 0;
        self.f.sign = (result & 0x80) != 0;
        self.f.carry = (a as u16 + b as u16) > 0xFF;
        self.f.half_carry = (a & 0x0F) + (b & 0x0F) > 0x0F;
        self.f.parity_overflow = ((a ^ !b) & (a ^ result) & 0x80) != 0;
        self.f.subtract = false;
    }

    fn update_flags_adc16(&mut self, a: u16, b: u16, result: u16) {
        self.f.zero = (result & 0xFFFF) == 0;
        self.f.sign = (result & 0x8000) != 0;
        self.f.carry = (a as u32 + b as u32 + self.f.carry as u32) > 0xFFFF;
        self.f.half_carry = (a & 0xFFF) + (b & 0xFFF) + self.f.carry as u16 > 0xFFF;
        self.f.parity_overflow = ((a ^ b ^ 0x8000) & (result ^ b) & 0x8000) != 0;
        self.f.subtract = false;
    }
}
