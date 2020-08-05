#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{fs, ops::Index};

#[derive(Copy, Clone)]
pub enum CartType {
    Rom = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBttry = 0x03,
    Mbc2 = 0x05,
    Mbc2RamBttry = 0x06,
    RomRam = 0x08,
    Mmm01 = 0x0B,
    Mmm01Ram = 0x0C,
    Mmm01RamBttry = 0x0D,
    Mbc3TimerBttry = 0x0F,
    Mbc3RamTimerBttry = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBttry = 0x13,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1A,
    Mbc5RamBttry = 0x1B,
    Mbc5Rumble = 0x1C,
    Mbc5RumbleBttry = 0x1E,
    Mbc6RamBttry = 0x20,
    Mbc7RamBttryAcclrmtr = 0x22,
    PocketCamera = 0xFc,
    BandaiTama5 = 0xFd,
    Huc3 = 0xFe,
    Huc1RamBttry = 0xFf,
}

impl From<u8> for CartType {
    // Necessary for conversion from u8 to enum
    fn from(val: u8) -> Self {
        match val {
            0x00 => CartType::Rom,
            0x01 => CartType::Mbc1,
            0x02 => CartType::Mbc1Ram,
            0x03 => CartType::Mbc1RamBttry,
            0x05 => CartType::Mbc2,
            0x06 => CartType::Mbc2RamBttry,
            0x08 => CartType::RomRam,
            0x0b => CartType::Mmm01,
            0x0c => CartType::Mmm01Ram,
            0x0d => CartType::Mmm01RamBttry,
            0x0F => CartType::Mbc3TimerBttry,
            0x10 => CartType::Mbc3RamTimerBttry,
            0x11 => CartType::Mbc3,
            0x12 => CartType::Mbc3Ram,
            0x13 => CartType::Mbc3RamBttry,
            0x19 => CartType::Mbc5,
            0x1A => CartType::Mbc5Ram,
            0x1B => CartType::Mbc5RamBttry,
            0x1C => CartType::Mbc5Rumble,
            0x1E => CartType::Mbc5RumbleBttry,
            0x20 => CartType::Mbc6RamBttry,
            0x22 => CartType::Mbc7RamBttryAcclrmtr,
            0xFc => CartType::PocketCamera,
            0xFd => CartType::BandaiTama5,
            0xFe => CartType::Huc3,
            0xFf => CartType::Huc1RamBttry,
            _ => {
                panic!("Could not understand cartridge type");
            }
        }
    }
}

// pub fn low_bit(x: u32) -> u8 {
//     (x & 0xFF) as u8
// }

// pub fn high_bit(x: u32) -> u8 {
//     ((x >> 8) & 0xFF) as u8
// }

#[allow(dead_code)]
pub struct Vram {
    //TODO rewrite to be able to use bank switching
    charram: [u8; 0x1800], /* 0x8000 - 0x97FF */
    bgdata1: [u8; 0x400],  /* 0x9800 - 0x9BFF */
    bgdata2: [u8; 0x400],  /* 0x9C00 - 0x9FFF */
}

impl Vram {
    pub fn write(&self, _addr: u16, _val: u8) {
        panic!("NOT IMPLEMENTED");
    }
    pub fn read(&self, _addr: u16) -> u8 {
        panic!("NOT IMPLEMENTED");
    }
}

pub struct CartHeader {
    //TODO use enum values for the ones that are applicable (necessary?)
    pub logo: Vec<u8>,
    pub title: String,
    pub manufact: String,
    pub gbc: bool,
    pub gbc_only: bool,
    pub new_license: String,
    pub sgb: bool,
    pub cart_type: CartType,
    pub size: u16,
    pub ramsize: u32,
    pub japan_code: u8,
    pub old_license: u8,
    pub use_new_license: bool,
    pub rom_version: u8,
    pub checksum: u8,
}

impl CartHeader {
    pub fn new(rom: &Vec<u8>) -> Self {
        let logo = rom.get(0x104..0x134).expect("Can not get logo").to_vec();

        let default_title: String = String::from("Default Title");
        let title = match rom.get(0x134..0x143) {
            Some(bytes) => String::from_utf8(Vec::from(bytes)).unwrap_or_else(|e| {
                error!("Can not convert title's bytes to String, using default title",);
                error!("Error: {}", e);
                default_title
            }),
            None => {
                error!("Can not get title values from rom, using default title",);
                default_title
            }
        };
        println!("loading the '{}'", title);
        let gbc_flag = rom.get(0x143).unwrap_or_else(|| {
            error!("Can not understand gbc_flag from rom, using default value");
            &(0xC0 as u8)
        });
        let mut gbc_only = false;
        let mut gbc = false;
        match *gbc_flag {
            0xC0 => {
                gbc_only = true;
                gbc = true;
            }
            0x80 => {
                gbc = true;
            }
            _ => {
                info!(
                    "GBC Flag '{}' from cartridge can not be understood, not GBC Rom",
                    *gbc_flag
                );
            }
        }
        let manufact = if gbc {
            match rom.get(0x13F..=0x142) {
                Some(bytes) => String::from_utf8(bytes.to_vec()).unwrap_or_else(|e| {
                    error!(
                        "Can not read manufact bytes to String, using default ''\n{}",
                        e
                    );
                    "".to_string()
                }),
                None => {
                    error!("Can not read bytes for manufact, using default ''");
                    "".to_string()
                }
            }
        } else {
            info!("No manufacturer code, game is not for GBC");
            '\0'.to_string()
        };

        let new_license = match rom.get(0x144..=0x145) {
            Some(bytes) => String::from_utf8(bytes.to_vec()).unwrap_or_else(|e| {
                error!(
                    "Can not read new license bytes to String, using default '00'\n{}",
                    e
                );
                "00".to_string()
            }),
            None => {
                error!("Can not read bytes for new license code, using default");
                "00".to_string()
            }
        };
        let sgb = *rom.get(0x146).expect("Can not read SGB flag from rom") == 0x3;
        let cart_type = *rom.get(0x147).expect("Can not read cart type value");
        let cart_type = CartType::from(cart_type);
        let mut specs = CartSpecs::default();
        match cart_type {
            CartType::Rom => specs.rom_only = true,
            CartType::Mbc1 => specs.mbc = 1,
            CartType::Mbc1Ram => {
                specs.mbc = 1;
                specs.ram = true
            }
            CartType::Mbc1RamBttry => {
                specs.mbc = 1;
                specs.ram = true;
                specs.battery = true
            }
            CartType::Mbc2 => specs.mbc = 2,
            CartType::Mbc2RamBttry => {
                specs.mbc = 2;
                specs.ram = true;
                specs.battery = true;
            }
            CartType::RomRam => specs.ram = true,
            CartType::Mmm01 => specs.mmm01 = true,
            CartType::Mmm01Ram => {
                specs.mmm01 = true;
                specs.ram = true;
            }
            CartType::Mmm01RamBttry => {
                specs.mmm01 = true;
                specs.ram = true;
                specs.battery = true;
            }
            CartType::Mbc3TimerBttry => {
                specs.mbc = 3;
                specs.timer = true;
                specs.battery = true;
            }
            CartType::Mbc3RamTimerBttry => {
                specs.mbc = 3;
                specs.timer = true;
                specs.battery = true;
                specs.ram = true;
            }
            CartType::Mbc3 => specs.mbc = 3,
            CartType::Mbc3Ram => {
                specs.mbc = 3;
                specs.ram = true;
            }
            CartType::Mbc3RamBttry => {
                specs.mbc = 3;
                specs.ram = true;
                specs.battery = true;
            }
            CartType::Mbc5 => specs.mbc = 5,
            CartType::Mbc5Ram => {
                specs.mbc = 5;
                specs.ram = true;
            }
            CartType::Mbc5RamBttry => {
                specs.mbc = 5;
                specs.ram = true;
                specs.battery = true;
            }
            CartType::Mbc5Rumble => {
                specs.mbc = 5;
                specs.rumble = true;
            }
            CartType::Mbc5RumbleBttry => {
                specs.mbc = 5;
                specs.rumble = true;
                specs.battery = true;
            }
            CartType::Mbc6RamBttry => {
                specs.mbc = 6;
                specs.ram = true;
                specs.battery = true;
            }
            CartType::Mbc7RamBttryAcclrmtr => {
                specs.mbc = 7;
                specs.ram = true;
                specs.battery = true;
                specs.accelerometer = true;
            }
            CartType::PocketCamera => specs.pocket_camera = true,
            CartType::BandaiTama5 => specs.bandai = true,
            CartType::Huc3 => specs.huc3 = true,
            CartType::Huc1RamBttry => {
                specs.huc1 = true;
                specs.battery = true;
                specs.ram = true;
            }
        }
        let size: u16 = match rom[0x148] {
            x if x < 9 => 0x8000 << x,
            0x52 | 0x53 | 0x54 => {
                panic!("This rom size is not supported at the moment");
            }
            _ => {
                error!("Rom size is not understood, using default rom size of 32Kb");
                32 * 1024
            }
        };

        let ramsize = match rom[0x149] {
            0 => 0, //None
            1 => 2 * 1024,
            2 => 8 * 1024,
            3 => 32 * 1024,
            4 => 128 * 1024,
            5 => 64 * 1024,
            _ => {
                error!("Ram size is not understood, using default ram size of 0 Kb");
                0
            }
        };
        let japan_code = rom[0x14A];
        let old_license = rom[0x14B];
        if old_license != 0x33 && sgb {
            panic!("Super GameBoy functions won't work");
        }
        let use_new_license = old_license == 0x33;
        if use_new_license {
            info!("Licensee code is {}", new_license);
        } else {
            info!("(Old) Licensee code is {}", old_license);
        }
        let rom_version = rom[0x14C];
        // Checksum function is x=0:FOR i=0134h TO 014Ch:x=x-MEM[i]-1:NEXT
        #[cfg(checksum)]
        {
            let mut x: u8 = 0;
            for i in 0x134..=0x14C {
                log_write(
                    LogLevel::Message,
                    &format!("Using value 0x{:x} for processing checksum", rom[i]),
                );
                x = x.wrapping_sub(rom[i].wrapping_sub(1));
            }

            let checksum = rom[0x14D];
            if (x & 0xff) != checksum {
                log_write(
                    LogLevel::Warning,
                    "Checksums doesnt match, ignoring checksums anyway",
                );
                log_write(
                    LogLevel::Warning,
                    &format!(
                        "Checksums:\n\tCalculated:0x{:x} Found:0x{:x}",
                        (x & 0xFF),
                        checksum
                    ),
                );
            } else {
                log_write(LogLevel::Message, "Checksums matches.");
            }
        }
        #[cfg(not(checksum))]
        let checksum = 0;

        // Ignoring global checksum altogether

        Self {
            logo,
            title,
            manufact,
            gbc,
            gbc_only,
            new_license,
            sgb,
            cart_type,
            size,
            ramsize,
            japan_code,
            old_license,
            use_new_license,
            rom_version,
            checksum,
        }
    }
}

#[derive(Default)]
struct CartSpecs {
    rom_only: bool,
    mbc: u8,
    battery: bool,
    ram: bool,
    mmm01: bool,
    timer: bool,
    rumble: bool,
    accelerometer: bool,
    pocket_camera: bool,
    bandai: bool,
    huc3: bool,
    huc1: bool,
}

// pub fn check_logo() {
//     let correct_logo = [
//         0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00,
//         0x0d, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9,
//         0x33, 0x3e,
//     ];
//     [> TODO <]
// }

pub struct Rom {
    rom: Vec<u8>,
    bank: u8,
}

impl From<Vec<u8>> for Rom {
    fn from(bytes: Vec<u8>) -> Self {
        Rom::new(bytes)
    }
}

impl Index<u16> for Rom {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        &self.read(index)
    }
}

impl Rom {
    pub fn new(rom: Vec<u8>) -> Self {
        Self { rom: rom, bank: 0 }
    }

    #[allow(dead_code)]
    pub fn set_bank(mut self, index: u8) {
        self.bank = index;
    }

    pub fn read(&self, addr: u16) -> &u8 {
        match addr {
            a @ 0x0000..=0x3FFF => &self.rom[a as usize],
            a => &self.rom[self.bank as usize * 0x4000 + a as usize],
        }
    }
}

pub struct Memory {
    rom: Rom,            /* 0x0000 - 0x8000 */
    vram: Vram,          /* 0x8000 - 0x9FFF */
    sram: [u8; 0x2000],  /* 0xA000 - 0xBFFF */
    wram0: [u8; 0x1000], /* 0xC000 - 0xCFFF */
    wramx: [u8; 0x1000], /* 0xD000 - 0xDFFF */
    oam: [u8; 0xa0],     /* 0xFE00 - 0xFE9F */
    ioregs: [u8; 0x80],  /* 0xFF00 - 0xFF7F */
    hram: [u8; 0x7f],    /* 0xFF80 - 0xFFFE */
    ie_reg: [u8; 0x1],   /* 0xFFFF */
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            rom: Rom::from(vec![0; 0x8000]),
            vram: Vram {
                charram: [0; 0x1800],
                bgdata1: [0; 0x400],
                bgdata2: [0; 0x400],
            },
            sram: [0; 0x2000],
            wram0: [0; 0x1000],
            wramx: [0; 0x1000],
            oam: [0; 0xa0],
            ioregs: [0; 0x80],
            hram: [0; 0x7f],
            ie_reg: [0; 0x1],
        }
    }
}

impl Memory {
    pub fn load_rom(&mut self, fname: &str) -> CartHeader {
        let rom_bytes = fs::read(fname).expect(&format!("Can not read rom file: {}", fname));
        info!("Number of bytes read from rom: {}", rom_bytes.len());
        self.rom = Rom::new(rom_bytes.clone());
        CartHeader::new(&rom_bytes)
    }

    pub fn read8(&self, addr: u16) -> u8 {
        let val = match addr {
            0x0000..=0x7fff => self.rom[addr],
            0x8000..=0x9FFF => self.vram.read(addr - 0x8000),
            0xA000..=0xBFFF => self.sram[(addr - 0xA000) as usize],
            0xC000..=0xCFFF => self.wram0[(addr - 0xC000) as usize],
            0xD000..=0xDFFF => self.wramx[(addr - 0xD000) as usize],
            0xe000..=0xFDFF => self.read8(addr - 0x2000),
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF7F => self.ioregs[(addr - 0xFF00) as usize],
            0xFF00..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie_reg[0],
        };
        info!("Value 0x{:x} read from 0x{:X}", val, addr);
        val
    }
    pub fn read16(&self, addr: u16) -> u16 {
        ((self.read8(addr + 1) as u16) << 8) | self.read8(addr) as u16
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7fff => {
                warn!("Ignoring write to ROM address {}", addr);
            }
            0x8000..=0x9FFF => {
                self.vram.write(addr - 0x8000, val);
            }
            0xA000..=0xBFFF => {
                self.sram[(addr - 0xA000) as usize] = val;
            }
            0xC000..=0xCFFF => {
                self.wram0[(addr - 0xC000) as usize] = val;
            }
            0xD000..=0xDFFF => {
                self.wramx[(addr - 0xD000) as usize] = val;
            }
            0xe000..=0xFDFF => {
                self.write(addr - 0x2000, val);
            }
            0xFE00..=0xFE9F => {
                self.oam[(addr - 0xFE00) as usize] = val;
            }
            0xFEA0..=0xFEFF => {
                info!(
                    "Ignoring write to unused range 0xFEA0-0xFEFF, requested address was 0x{:X}",
                    addr
                );
                return;
            }
            0xFF00..=0xFF7F => {
                self.ioregs[(addr - 0xFF00) as usize] = val;
            }
            0xFF00..=0xFFFE => {
                self.hram[(addr - 0xFF80) as usize] = val;
            }
            0xFFFF => {
                self.ie_reg[0] = val;
            }
        }
        info!("0x{:x} written to memory address 0x{:X}", val, addr);
    }

    pub fn write16(&mut self, addr: u16, val: u16) {
        let ls_byte = val as u8 & 0xff;
        let ms_byte = (val >> 8) as u8;
        self.write(addr, ls_byte);
        self.write(addr + 1, ms_byte);
    }
}
