use clap::{crate_authors, crate_version, App, Arg};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use simplelog::{LevelFilter, WriteLogger};
use std::{env, io::stdout};

mod cpu;
use cpu::Cpu;
mod mem;
use mem::Memory;

fn main() {
    let matches = App::new("ruBoy")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A Gameboy emulator written in Rust")
        .arg(
            Arg::with_name("log")
                .short("l")
                .long("log-level")
                .value_name("LEVEL")
                .possible_values(&["off", "error", "warn", "debug", "info", "trace", "all"])
                .default_value("all")
                .number_of_values(1)
                .help("The verbosity level of logs"),
        )
        .arg(
            Arg::with_name("rom")
                .help("Set the rom file to use")
                .required(true)
                .index(1),
        )
        .get_matches();
    let log_level = matches.value_of("log").unwrap_or("warn");
    let log_level = match log_level {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Trace,
    };
    WriteLogger::init(log_level, simplelog::Config::default(), stdout()).unwrap();

    let romname = matches.value_of("rom").expect("Rom file need to be specified");
    let mut memory = Memory::default();
    let cartridge = memory.load_rom(&romname);
    let mut cpu = Cpu::new(memory);
    cpu.run(cartridge);
}
