struct Flags {
    carry: bool,
    zero: bool,
    sign: bool,
    parity_overflow: bool,
    half_carry: bool,
    subtract: bool,
}

struct Z80 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: Flags,
    pc: u16,
    sp: u16,
    ix: u16,
    iy: u16,
    iff1: bool,
    iff2: bool,
    memory: Memory,
}

impl Z80 {
    fn new() -> Self {
        Z80 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: Flags {
                carry: false,
                zero: false,
                sign: false,
                parity_overflow: false,
                half_carry: false,
                subtract: false,
            },
            pc: 0,
            sp: 0,
            ix: 0,
            iy: 0,
            iff1: false,
            iff2: false,
            memory: Memory::new(),
        }
    }

    // Helper methods to combine 8-bit registers into 16-bit pairs
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
}
