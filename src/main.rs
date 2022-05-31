use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod bus;
mod cpu;
mod dram;
use crate::cpu::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: rvemu-for-book <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);

    loop {
        // FETCH
        let inst = match cpu.fetch() {
            // BREAK ON ERR
            Ok(inst) => inst,
            Err(_) => break,
        };

        // INCREMENT PC
        cpu.pc += 4;

        // DECODE AND EXECUTE
        match cpu.execute(inst) {
            Ok(_) => {}
            Err(_) => break,
        }

        if cpu.pc == 0 {
            break;
        }
    }

    cpu.show_registers();

    Ok(())
}
