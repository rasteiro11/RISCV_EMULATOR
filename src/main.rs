use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;

// A REPRESENTATION OF CPU
struct Cpu {
    // 32 x 64BIT REGISTERS
    regs: [u64; 32],
    // PROGRAM COUNTER STORES THE NEXT ADDRESS OF DRAM TO BE EXECUTED
    pc: u64,
    // DRAM STORES EXECUTABLE INSTRUCTIONS
    dram: Vec<u8>,
}

impl Cpu {
    fn new(code: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = DRAM_SIZE;

        Self {
            regs,
            pc: 0,
            dram: code,
        }
    }

    pub fn show_registers(&self) {
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        for i in 0..32 {
            println!("{} {}", abi[i], self.regs[i]);
        }
    }

    // GET A 32BIT INSRUCTION FROM DRAM
    fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
    }

    /// Execute an instruction after decoding.
    fn execute(&mut self, inst: u32) {
        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;

        // ZERO REGISTER NEEDS TO BE ALWAYS 0
        self.regs[0] = 0;

        match opcode {
            0x13 => {
                // addi
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
            _ => {
                // dbg!(format!("not implemented yet: opcode {:#x}", opcode));
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: rvemu-for-book <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);

    while cpu.pc < cpu.dram.len() as u64 {
        // 1. Fetch.
        let inst = cpu.fetch();

        // 2. Add 4 to the program counter.
        cpu.pc += 4;

        // 3. Decode.
        // 4. Execute.
        cpu.execute(inst);
    }
    // cpu.dump_registers();
    //println!("VALUE: {}", cpu.regs[31]);
    cpu.show_registers();

    Ok(())
}
