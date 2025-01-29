struct Psg {
    channels: [PsgChannel; 4],
    latched_channel: usize,
}

struct PsgChannel {
    volume: u8,
    frequency: u16,
    counter: u16,
}

impl Psg {
    fn new() -> Self {
        Psg {
            channels: [
                PsgChannel { volume: 0, frequency: 0, counter: 0 },
                PsgChannel { volume: 0, frequency: 0, counter: 0 },
                PsgChannel { volume: 0, frequency: 0, counter: 0 },
                PsgChannel { volume: 0, frequency: 0, counter: 0 },
            ],
            latched_channel: 0,
        }
    }

    fn write(&mut self, value: u8) {
        if (value & 0x80) != 0 {
            
            self.latched_channel = ((value >> 5) & 0x03) as usize;
            let is_volume = (value & 0x10) != 0;
            
            if is_volume {
                self.channels[self.latched_channel].volume = value & 0x0F;
            } else {
                self.channels[self.latched_channel].frequency =
                    (self.channels[self.latched_channel].frequency & 0x3F0) | 
                    (value & 0x0F) as u16;
            }
        } else {
           
            if self.latched_channel < 3 {
                self.channels[self.latched_channel].frequency =
                    (self.channels[self.latched_channel].frequency & 0x00F) | 
                    ((value as u16) << 4);
            }
        }
    }
}
