struct Vdp {
    vram: [u8; 0x4000],  // 16KB VRAM
    registers: [u8; 16],  // VDP registers
    status: u8,
    address_latch: u16,
    control_code: u8,
    interrupt_pending: bool,
}

impl Vdp {
    fn new() -> Self {
        Vdp {
            vram: [0; 0x4000],
            registers: [0; 16],
            status: 0,
            address_latch: 0,
            control_code: 0,
            interrupt_pending: false,
        }
    }

    fn write_port(&mut self, port: u8, value: u8) {
        match port {
            0xBE => self.write_data(value),
            0xBF => self.write_control(value),
            _ => (),
        }
    }

    fn write_control(&mut self, value: u8) {
        if self.control_code == 0 {
            self.address_latch = (self.address_latch & 0xFF00) | value as u16;
            self.control_code = 1;
        } else {
            self.address_latch = (self.address_latch & 0x00FF) | ((value as u16 & 0x3F) << 8);
            self.control_code = 0;
            if (value & 0x80) != 0 {
                self.write_register(value & 0x0F, (self.address_latch >> 8) as u8);
            }
        }
    }

    fn write_data(&mut self, value: u8) {
        self.vram[self.address_latch as usize] = value;
        self.address_latch = (self.address_latch + 1) & 0x3FFF;
    }

    fn write_register(&mut self, reg: u8, value: u8) {
        self.registers[reg as usize] = value;
        match reg {
            1 => self.update_interrupt_state(),
            _ => (),
        }
    }

    fn update_interrupt_state(&mut self) {
        self.interrupt_pending = (self.registers[1] & 0x20) != 0;
    }
}
