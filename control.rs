impl Z80 {
    
    fn jp_nn(&mut self) {
        let nn = self.fetch_word();
        self.pc = nn;
    }
 
    fn ret_nz(&mut self) {
        if !self.f.zero {
            let addr = self.memory.read_word(self.sp);
            self.sp = self.sp.wrapping_add(2);
            self.pc = addr;
        }
    }

    fn call_nn(&mut self) {
        let nn = self.fetch_word();
        self.sp = self.sp.wrapping_sub(2);
        self.memory.write_word(self.sp, self.pc);
        self.pc = nn;
    }
}
