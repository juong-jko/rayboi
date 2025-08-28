mod cpu;
mod registers;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::cpu::{CPU, Instruction, ArithmeticTarget};

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        cpu.registers.a = 5;
        cpu.registers.c = 10;
        cpu.execute(Instruction::Add(ArithmeticTarget::C));
        assert_eq!(cpu.registers.a, 15);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.carry, false);
        assert_eq!(cpu.registers.f.half_carry, false);
    }
}