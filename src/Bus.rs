mod cpu_6502;

pub struct Bus {
  // devices
  pub cpu: cpu_6502::Cpu6502;
  
}

impl Bus {
  fn write(addr: u16, data: u8) {}
}
impl Bus {
  fn read(addr: u16, is_read_only: bool = false) -> u8 {}
}
