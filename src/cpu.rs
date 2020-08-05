use crate::mem::{CartHeader, Memory};
use log::{warn, info, error, debug, trace};
use std::fmt::{self, Debug, Formatter};

#[derive(Default, PartialEq)]
struct RegPair(u8, u8);

impl RegPair {
    pub fn set(&mut self, val: u16) {
        self.0 = (val >> 8) as u8;
        self.1 = (val & 0xff) as u8;
    }
    pub fn get(&self) -> u16 {
        ((self.0 as u16) << 8) | (self.1 as u16)
    }
}

impl From<u16> for RegPair {
    fn from(val: u16) -> Self {
        Self((val >> 8) as u8, (val & 0xff) as u8)
    }
}

pub struct Cpu {
    af: RegPair,
    bc: RegPair,
    de: RegPair,
    hl: RegPair,
    sp: u16,
    pc: u16,
    cycle: u64,
    mem: Memory,
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("CPU")
            .field("AF", &self.af.get())
            .field("BC", &self.bc.get())
            .field("DE", &self.de.get())
            .field("HL", &self.hl.get())
            .field("SP", &self.sp)
            .field("PC", &self.pc)
            .field("cycle count", &self.cycle)
            .finish()
    }
}

const OP_CYCLES: [u8; 0x100] = [
  /* 0   1    2   3   4   5   6   7   8   9   a  b   c   d  e   f */
      4, 12,  8,  8,  4,  4,  8,  4, 20,  8,  8, 8,  4,  4, 8,  4, //0
      4, 12,  8,  8,  4,  4,  8,  4, 12,  8,  8, 8,  4,  4, 8,  4, //1
      8, 12,  8,  8,  4,  4,  8,  4, 12,  8,  8, 8,  4,  4, 8,  4, //2
      8, 12,  8,  8, 12, 12, 12,  4,  8,  8,  8, 8,  4,  4, 8,  4, //3
      4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4, 4,  4,  4, 8,  4, //4
      4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4, 4,  4,  4, 8,  4, //5
      4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4, 4,  4,  4, 8,  4, //6
      8,  8,  8,  8,  8,  8,  4,  8,  4,  4,  4, 4,  4,  4, 8,  4, //7
      4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4, 4,  4,  4, 8,  4, //8
      4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4, 4,  4,  4, 8,  4, //9
      4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4, 4,  4,  4, 8,  4, //a
      4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4, 4,  4,  4, 8,  4, //b
      8, 12, 12, 16, 12, 16,  8, 16,  8, 16, 12, 4, 12, 24, 8, 16, //c
      8, 12, 12,  0, 12, 16,  8, 16,  8, 16, 12, 0, 12,  0, 8, 16, //d
     12, 12,  8,  0,  0, 16,  8, 16, 16,  4, 16, 0,  0,  0, 8, 16, //e
     12, 12,  8,  4,  0, 16,  8, 16, 12,  8, 16, 4,  0,  0, 8, 16, //f
];

impl Cpu {
    pub fn new(mem: Memory) -> Self {
        Self {
            af: RegPair::default(),
            bc: RegPair::default(),
            de: RegPair::default(),
            hl: RegPair::default(),
            sp: 0,
            pc: 0,
            cycle: 0,
            mem: mem,
        }
    }

    fn set_flag_zero(&mut self, val: bool){
        if val {
            self.af.1 |= 1<<7;
        }
        else {
            self.af.1 &= !(1<<7);
        }
    }
    fn get_flag_zero(&self) -> bool {
        self.af.1 & 1 << 7 != 0
    }
    fn set_flag_substract(&mut self, val: bool){
        if val {
            self.af.1 |= 1<<6;
        }
        else {
            self.af.1 &= !(1<<6);
        }
    }
    fn get_flag_substract(&self) -> bool {
        self.af.1 & 1 << 6 != 0
    }
    fn set_flag_half_carry(&mut self, val: bool){
        if val {
            self.af.1 |= 1<<5;
        }
        else {
            self.af.1 &= !(1<<5);
        }
    }
    fn get_flag_half_carry(&self) -> bool {
        self.af.1 & 1 << 5 != 0
    }
    fn set_flag_carry(&mut self, val: bool){
        if val {
            self.af.1 |= 1 << 4;
        }
        else {
            self.af.1 &= !(1<<4);
        }
    }
    fn get_flag_carry(&self) -> bool {
        self.af.1 & 1 << 4 != 0
    }

    fn initialize(&mut self, cart: &CartHeader) {
        if cart.gbc || cart.gbc_only {
            self.af.set(0x1180);
            self.bc.set(0x0000);
            self.de.set(0xff56);
            self.hl.set(0x000d);
        } else {
            self.af.set(0x01b0);
            self.bc.set(0x0013);
            self.de.set(0x00d8);
            self.hl.set(0x014d);
        }
        self.sp = 0xFFFE;
        self.pc = 0x0100;
    }

    pub fn run(&mut self, cart: CartHeader) {
        self.initialize(&cart);
        loop {
            let op = self.fetch();
            info!("Op code is 0x{:x}.", op);
            info!("{:04x?}", self);
            self.execute(op);
        }
    }

    fn execute(&mut self, op: u8) {
        self.cycle += OP_CYCLES[op as usize] as u64;
        match op {
            0x0 => {
                /* NOP */
            }
            0x01 => {
                /* LD BC,d16 */
                let d16 = self.fetch16();
                self.bc.set(d16)
            }
            // 0x02 => {
            //     [>LD (BC), A <]


            // }
            0x19 => {
                /*  ADD HL, DE 
                 *    1  8 
                 *  - 0 H C */
                let (res, carry) = self.de.get().overflowing_add(self.hl.get());
                let (_, half_carry) = self.hl.1.overflowing_add(self.de.1);
                self.hl.set(res);
                self.set_flag_substract(false);
                self.set_flag_half_carry(half_carry);
                self.set_flag_carry(carry);
            }
            0x21 => {
                /* LD HL, d16 */
                let a16 = self.fetch16();
                self.hl.set(a16);
            }
            0xc3 => {
                /* JP a16 */
                let a16 = self.fetch16();
                self.jump(a16);
            }
            0xcd => {
                /* CALL a16 */
                let a16 = self.fetch16();
                self.stack_push16(self.pc);
                self.jump(a16);
            }
            // 0xfa => {
                /* LD A, (a16)
                 *  3  16 */

            // }
            0xe0 => {
                /* LDH (a8), A 
                 *   2  12 
                 */
                let a8 = self.fetch();
                let addr = 0xFF00 + a8 as u16;
                self.mem.write(addr, self.af.0);
            }
            0xf0 => {
                /*LDH A, (a8) 
                 *  2  12 
                 */
                let a8 = self.fetch();
                let val = self.mem.read8(0xFF00 + a8 as u16);
                self.af.0 = val;
            }
            _ => panic!("This op code is not supported yet! opcode: 0x{:x}", op),
        }
    }

    fn stack_push16(&mut self, val: u16) {
        self.sp -= 2;
        self.mem.write16(self.sp, val);
    }

    fn fetch(&mut self) -> u8 {
        let val = self.mem.read8(self.pc);
        self.pc += 1;
        val
    }

    fn fetch16(&mut self) -> u16 {
        let val = self.mem.read16(self.pc);
        self.pc += 2;
        val
    }

    fn jump(&mut self, addr: u16) {
        trace!("Jumping to 0x{:X}", addr);
        self.pc = addr;
    }
}

// struct Interrupts<'a> {
//     master: bool,
//     flags: &'a u8,
//     enable: &'a u8,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env::{self},
        fs::File,
        io::{self, Write},
    };

    fn create_rom_file(bytes: Vec<u8>) -> std::io::Result<String> {
        let filepath = env::temp_dir().join("uboy_tmp_gameboy_rom.gb");
        let mut file = File::create(filepath.clone())?;
        file.write_all(&bytes)?;
        Ok(String::from(filepath.to_str().unwrap()))
    }

    #[test]
    fn nop() {
        let mem = Memory::default();
        let mut cpu = Cpu::new(mem);
        cpu.execute(0x0);
        assert!(cpu.af == RegPair::from(0));
        assert!(cpu.bc == RegPair::from(0));
        assert!(cpu.de == RegPair::from(0));
        assert!(cpu.hl == RegPair::from(0));
        assert!(cpu.sp == 0);
        assert!(cpu.pc == 1);
        assert!(cpu.cycle == 4);
    }

    #[test]
    fn ld_bc_d16() -> io::Result<()> {
        let mem = Memory::default();
        let mut cpu = Cpu::new(mem);
        cpu.mem
            .load_rom(&create_rom_file(vec![0x1u8, 0xfu8, 0xeu8])?);
        cpu.execute(0x1);
        assert!(cpu.af == RegPair::from(0));
        assert!(cpu.bc == RegPair::from((0xf << 8) | 0xe));
        assert!(cpu.de == RegPair::from(0));
        assert!(cpu.hl == RegPair::from(0));
        assert!(cpu.sp == 0);
        assert!(cpu.pc == 3);
        assert!(cpu.cycle == 12);
        Ok(())
    }

    #[test]
    fn ld_bc_a() -> io::Result<()> {
        let mem = Memory::default();
        let mut cpu = Cpu::new(mem);
        cpu.mem
            .load_rom(&create_rom_file(vec![0x2u8, 0xfu8, 0xeu8])?);
        cpu.bc = RegPair::from(0xC000);
        cpu.af.0 = 0xf;
        cpu.execute(0x1);
        assert!(cpu.af == RegPair::from(0xf00));
        assert!(cpu.bc == RegPair::from(0xc000));
        assert!(cpu.de == RegPair::from(0));
        assert!(cpu.hl == RegPair::from(0));
        assert!(cpu.sp == 0);
        assert!(cpu.pc == 3);
        assert!(cpu.cycle == 12);
        assert!(cpu.mem.read8(0xC00) == cpu.af.0);
        Ok(())
    }
}
