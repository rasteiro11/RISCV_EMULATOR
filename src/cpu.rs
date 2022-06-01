use crate::bus::*;
use crate::dram::*;

// A REPRESENTATION OF CPU
pub struct Cpu {
    // 32 x 64BIT REGISTERS
    pub regs: [u64; 32],
    // PROGRAM COUNTER STORES THE NEXT ADDRESS OF DRAM TO BE EXECUTED
    pub pc: u64,
    // SYSTEM BUS TRANSFERS DATA BETWEEN CPU AND PERIPHERAL DEVICES
    pub bus: Bus,
}

impl Cpu {
    pub fn new(code: Vec<u8>) -> Self {
        // STACK POINTER MUST BE SET
        let mut regs = [0; 32];
        regs[2] = DRAM_BASE + DRAM_SIZE;

        Self {
            regs,
            pc: DRAM_BASE,
            bus: Bus::new(code),
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

    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, ()> {
        self.bus.load(addr, size)
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        self.bus.store(addr, size, value)
    }

    pub fn fetch(&mut self) -> Result<u64, ()> {
        match self.bus.load(self.pc, 32) {
            Ok(inst) => Ok(inst),
            Err(_e) => Err(()),
        }
    }

    // GET A 32-BIT INSRUCTION FROM DRAM
    // fn fetch(&self) -> u32 {
    //     let index = self.pc as usize;
    //     return (self.dram[index] as u32)
    //         | ((self.dram[index + 1] as u32) << 8)
    //         | ((self.dram[index + 2] as u32) << 16)
    //         | ((self.dram[index + 3] as u32) << 24);
    // }

    /// Execute an instruction after decoding.
    fn execute(&mut self, inst: u32) -> Result<(), ()> {
        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
        let func3 = (inst >> 12) & 0x7;
        let func7 = (inst >> 25) & 0x7f;

        // ZERO REGISTER NEEDS TO BE ALWAYS 0
        self.regs[0] = 0;

        match opcode {
            0x03 => {
                let imm = ((inst as i32 as i64) >> 20) as u64;
                let addr = self.regs[rs1].wrapping_add(imm);
                match func3 {
                    0x0 => {
                        // LB
                        let val = self.load(addr, 8)?;
                        self.regs[rd] = val as i8 as i64 as u64;
                    }
                    0x1 => {
                        // LH
                        let val = self.load(addr, 16)?;
                        self.regs[rd] = val as i16 as i64 as u64;
                    }
                    0x2 => {
                        // LW
                        let val = self.load(addr, 32)?;
                        self.regs[rd] = val as i32 as i64 as u64;
                    }
                    0x3 => {
                        // LD
                        let val = self.load(addr, 64)?;
                        self.regs[rd] = val;
                    }
                    0x4 => {
                        // LBU
                        let val = self.load(addr, 8)?;
                        self.regs[rd] = val;
                    }
                    0x5 => {
                        // LHU
                        let val = self.load(addr, 16)?;
                        self.regs[rd] = val;
                    }
                    0x6 => {
                        // LWU
                        let val = self.load(addr, 32)?;
                        self.regs[rd] = val;
                    }

                    _ => {
                        return Err(());
                    }
                }
            }
            0x13 => {
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                // THE SHAMT IS ENCODED IN THE LOWER 6 BITS"
                let shamt = (imm & 0x3f) as u32;
                match func3 {
                    0x0 => {
                        // ADDI
                        self.regs[rd] = self.regs[rs1].wrapping_add(imm);
                    }
                    0x1 => {
                        // SLLI
                        self.regs[rd] = self.regs[rs1] << shamt;
                    }
                    0x2 => {
                        self.regs[rd] = if (self.regs[rs1] as i64) < (imm as i64) {
                            1
                        } else {
                            0
                        };
                    }
                    0x3 => {
                        // SLTIU
                        self.regs[rd] = if self.regs[rs1] < imm { 1 } else { 0 };
                    }
                    0x4 => {
                        // XORI
                        self.regs[rd] = self.regs[rs1] ^ imm;
                    }
                    0x5 => {
                        match func7 >> 1 {
                            // SRLI
                            0x00 => self.regs[rd] = self.regs[rs1].wrapping_shr(shamt),
                            // SRAI
                            0x10 => {
                                self.regs[rd] = (self.regs[rs1] as i64).wrapping_shr(shamt) as u64
                            }
                            _ => {}
                        }
                    }
                    // ORI
                    0x6 => self.regs[rd] = self.regs[rs1] | imm,
                    // ANDI
                    0x7 => self.regs[rd] = self.regs[rs1] & imm,
                    _ => {}
                }
            }
            0x17 => {
                // AUI PC
                let imm = (inst & 0xfffff000) as i32 as i64 as u64;
                self.regs[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
        }
        return Err(());
    }
}
