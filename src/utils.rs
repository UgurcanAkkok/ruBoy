use std::env;
// use std::fs::File;

// pub const LOG_FILE_NAME: &str = "log.txt";
// pub const LOG_ENABLE: bool = true;
// pub const DEBUG_ON: bool = true;

pub enum LogLevel {
    Message,
    Warning,
    Critical,
    Error,
}

// pub static log_lvl: LogLevel = LogLevel::Message;
// pub static log_file: Option<File> = None;
// pub fn disassemble(fname: &str) {
//     //TODO
// }

pub fn log_init() {
    //TODO
}
pub fn log_close() {
    //TODO
}
pub fn usage() {
    println!(
        "usage is: {}, ROM_FILE",
        env::args().next().unwrap_or(String::from("uboy"))
    );
}

pub fn set_log_lvl(lvl: LogLevel) {
    //TODO
}

pub fn log_write(lvl: LogLevel, log_str: &str) {
    // TODO print according to current log level
    println!("{}",log_str);
}
