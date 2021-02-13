mod op_codes;
mod bus;
mod cpu;

fn main() {
    let mut bus = bus::MemoryBus::new();
    let mut cpu = cpu::CPU::new(&mut bus);

    cpu.clock();
    cpu.clock();
    cpu.clock();
    cpu.clock();
    cpu.clock();
    cpu.clock();
}
