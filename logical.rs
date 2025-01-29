impl Z80 {
    // AND A, n
    fn and_a_n(&mut self) {
        let n = self.fetch_byte();
        self.a &= n;
        self.f.zero = self.a == 0;
        self.f.sign = (self.a & 0x80) != 0;
        self.f.half_carry = true; // AND sets H flag
        self.f.parity_overflow = self.a.count_ones() % 2 == 0;
        self.f.subtract = false;
        self.f.carry = false;
    }

    // XOR A, B
    fn xor_a_b(&mut self) {
        self.a ^= self.b;
        self.f.zero = self.a == 0;
        self.f.sign = (self.a & 0x80) != 0;
        self.f.half_carry = false;
        self.f.parity_overflow = self.a.count_ones() % 2 == 0;
        self.f.subtract = false;
        self.f.carry = false;
    }
}
