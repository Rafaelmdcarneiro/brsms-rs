struct Memory {
    rom: Vec<u8>,
    ram: [u8; 0x2000], // 8KB RAM
}

impl Memory {
    fn new() -> Self {
        Memory {
            rom: Vec::new(),
            ram: [0; 0x2000],
        }
    }

    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0xBFFF => self.rom.get(addr as usize).copied().unwrap_or(0),
            0xC000..=0xFFFF => self.ram[(addr as usize - 0xC000) % 0x2000],
            _ => 0,
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        if let 0xC000..=0xFFFF = addr {
            self.ram[(addr as usize - 0xC000) % 0x2000] = value;
        }
    }

    fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr + 1) as u16;
        (hi << 8) | lo
    }

    fn write_word(&mut self, addr: u16, value: u16) {
        let lo = (value & 0xFF) as u8;
        let hi = (value >> 8) as u8;
        self.write_byte(addr, lo);
        self.write_byte(addr + 1, hi);
    }
}
