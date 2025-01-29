impl Z80 {
    
    fn rlc_a(&mut self) {
        let carry = (self.a & 0x80) != 0;
        self.a = self.a.rotate_left(1);
        self.f.carry = carry;
        self.f.zero = self.a == 0;
        self.f.subtract = false;
        self.f.half_carry = false;
    }

    fn sra_b(&mut self) {
        let sign = self.b & 0x80;
        let carry = (self.b & 0x01) != 0;
        self.b = (self.b >> 1) | sign;
        self.f.carry = carry;
        self.f.zero = self.b == 0;
        self.f.sign = (self.b & 0x80) != 0;
        self.f.parity_overflow = self.b.count_ones() % 2 == 0;
        self.f.subtract = false;
        self.f.half_carry = false;
    }
}
