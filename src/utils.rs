use std::env;
// use std::fs::File;

// pub const LOG_FILE_NAME: &str = "log.txt";
// pub const LOG_ENABLE: bool = true;
// pub const DEBUG_ON: bool = true;
// pub fn disassemble(fname: &str) {
//     //TODO
// }
pub fn usage() {
    println!(
        "usage is: {}, ROM_FILE",
        env::args().next().unwrap_or(String::from("uboy"))
    );
}
