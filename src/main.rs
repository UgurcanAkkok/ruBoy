use std::{env, io::stdout};
use log::{warn, info, error, debug, trace};
use simplelog::{WriteLogger, LevelFilter};
mod cpu;
mod mem;
mod utils;
use utils::{usage};
use mem::Memory;
use cpu::Cpu;

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        usage();
        return;
    }
    args.next();
    let mut romname = String::default();
    let mut dump = false;
    loop {
        match args.next() {
            Some(arg) => {
                if arg.starts_with("-h") {
                    usage();
                }
                if arg.starts_with("-l") {
                    //TODO set log level
                }
                if arg.starts_with("-x") {
                    dump = true;
                } else {
                    romname = arg;
                }
            }
            None => break,
        }
    }
    WriteLogger::init(LevelFilter::Off, simplelog::Config::default(), stdout());

    if dump {
        // disassemble(&romname);
    } else {
        let mut memory = Memory::default();
        let cartridge = memory.load_rom(&romname);
        let mut cpu = Cpu::new(memory);
        cpu.run(cartridge);
    }
}
