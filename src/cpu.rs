use crate::{bus, op_codes};

struct Registers {
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    a: u8,
    f: u8,
}

pub(crate) struct CPU<'bus, T: bus::Bus> {
    registers: Registers,
    pc: u16,
    sp: u16,
    wait: u8,
    bus: &'bus mut T,
}

impl <'bus, T: bus::Bus> CPU<'bus, T> {
    pub fn new(bus: &'bus mut T) -> CPU<'bus, T> {
        CPU {
            registers: Registers {
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
                a: 0,
                f: 0,
            },
            pc: 0,
            sp: 0,
            wait: 0,
            bus,
        }
    }

    pub fn clock(&mut self) {
        if self.wait == 0 {
            // Read data from PC
            let data = self.bus.read(self.pc);

            // Decode instruction
            let op_code = &op_codes::OP_CODES[usize::from(data)];

            println!("[${}] Fetched code {} [C: {}, L: {}] from position {}", self.pc, op_code.mnemonic, op_code.cycles, op_code.length, usize::from(data));

            self.wait = self.wait + op_code.cycles;
            self.pc = self.pc + op_code.length;
        } else {
            println!("CPU is waiting...");
        }

        self.wait = self.wait - 1;
    }
}