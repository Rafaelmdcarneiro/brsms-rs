struct Input {
    port_a: u8,  // Controller 1 (0xDC)
    port_b: u8,  // Controller 2 (0xDD)
    port_sel: u8,
}

impl Input {
    fn new() -> Self {
        Input {
            port_a: 0xFF,
            port_b: 0xFF,
            port_sel: 0,
        }
    }

    fn read_port(&self, port: u8) -> u8 {
        match port {
            0xDC => self.port_a,
            0xDD => self.port_b,
            _ => 0xFF,
        }
    }

    fn set_button_state(&mut self, port: u8, button: Button, pressed: bool) {
        let mask = match button {
            Button::Up => 0x01,
            Button::Down => 0x02,
            Button::Left => 0x04,
            Button::Right => 0x08,
            Button::Button1 => 0x10,
            Button::Button2 => 0x20,
        };

        let value = if pressed { !mask } else { 0xFF };
        match port {
            0 => self.port_a &= value,
            1 => self.port_b &= value,
            _ => (),
        }
    }
}

enum Button {
    Up,
    Down,
    Left,
    Right,
    Button1,
    Button2,
}
