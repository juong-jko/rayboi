enum Instruction {
    Add(ArithmeticTarget),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

struct CPU {
    registers: Registers
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(target) => {
                match target {
                    ArithmeticTarget::C => {
                        // Read current value from the target register
                        let value = self.Registers.c;
                        // Add the value to the value in the A register
                        let new_value = self.add(value);
                        // Write updated value to the A register
                        self.Registers.a = new_value;
                    }
                }
            }
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
}

