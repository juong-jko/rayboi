use crate::registers::{Registers, FlagsRegister};
pub enum Instruction {
    Add(ArithmeticTarget),
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub bus: MemoryBus,
}

pub struct MemoryBus {
    mem : [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers { a:0, b: 0, c: 0, d: 0, e: 0, f: FlagsRegister { zero: false, subtract: false, half_carry: false, carry: false }, h: 0, l: 0 },
            pc: 0,
            bus: MemoryBus { mem: [0; 0xFFFF] }
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Add(target) => {
                match target {
                    ArithmeticTarget::C => {
                        // Read current value from the target register
                        let value = self.registers.c;
                        // Add the value to the value in the A register
                        let new_value = self.add(value);
                        // Write updated value to the A register
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => self.pc
                }
            }
            _ => self.pc
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        // does not panic when the addition overflows
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the value and register A
        // results in a value bigger than 0xF
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    // fn step(&mut self) {
    //     let instruction_byte = self.bus.read_byte(self.pc);

    //     let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
    //         self.execute(instruction)
    //     } else {
    //         panic!("Unknown instruction found for 0x{:x}", instruction_byte);
    //     };

    //     self.pc = next_pc;
    // }
}

// impl Instruction {
//     fn from_byte(byte: u8) -> Option<Instruction> {
//         match byte {
//             0x02 => Some(Instruction::INC(IncDecTarget::BC)),
//             0x13 => Some(Instruction::INC(IncDecTarget::DE)),
//             _ => None
//         }
//     }
// }