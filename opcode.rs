impl Z80 {
    fn execute_next(&mut self) {
        let opcode = self.fetch_byte();
        match opcode {
           
            0x00 => self.nop(),
            0x01 => self.ld_bc_nn(),
            0x80 => self.add_a_b(),
            0xC3 => self.jp_nn(),
            0xCD => self.call_nn(),
            0xFE => self.cp_n(),

            0xCB => self.execute_cb_opcode(),
            0xED => self.execute_ed_opcode(),
            0xDD => self.execute_ix_opcode(), 
            0xFD => self.execute_iy_opcode(), 

            _ => panic!("Unimplemented opcode: 0x{:02X}", opcode),
        }
    }
}
