pub trait Bus {
    fn write(&mut self, address: u16, data: u8);
    fn read(&mut self, address: u16) -> u8;
}

pub struct MemoryBus {
    memory: [u8; 0x10000]
}

impl Bus for MemoryBus {
    fn write(&mut self, address: u16, data: u8) {
        self.memory[usize::from(address)] = data
    }

    fn read(&mut self, address: u16) -> u8 {
        self.memory[usize::from(address)]
    }
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus { memory: [0x00; 0x10000] }
    }
}