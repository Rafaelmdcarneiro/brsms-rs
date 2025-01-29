struct Memory {
    rom: Vec<u8>,
    ram: [u8; 0x2000],
    vdp: Vdp,
    psg: Psg,
    input: Input,
}

impl Memory {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // ROM
            0x0000..=0xBFFF => self.rom.get(addr as usize).copied().unwrap_or(0),
            
            // VDP VRAM access
            0xC000..=0xFFFF => self.vdp.vram[(addr - 0xC000) as usize],
            
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM
            0xC000..=0xDFFF => self.ram[(addr - 0xC000) as usize] = value,
            
            // VDP ports
            0xBE => self.vdp.write_port(0xBE, value),
            0xBF => self.vdp.write_port(0xBF, value),
            
            // PSG port
            0x7F => self.psg.write(value),
            
            _ => (),
        }
    }

    fn read_port(&self, port: u8) -> u8 {
        match port {
            0xDC | 0xDD => self.input.read_port(port),
            _ => 0xFF,
        }
    }
}
