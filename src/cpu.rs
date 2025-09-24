pub struct CPU {
    pub register_a: u8, // accumulator CPU register (temp storage of data for calc)
    pub register_x: u8, // another register to store data
    pub register_y: u8, // another register to store data
    pub status: u8,     // processor status register
    pub program_counter: u16, // the pc (keeps track of our curr pos in the program)
}

impl CPU {
    /// Instantiates the CPU (all set to 0)
    pub fn new() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
        }
    }

    /// ADC - Add with Carry
    ///
    /// Increments the accumulator register with the content of a memory location
    /// in combination with the carry bit. If unsigned overflow occurs, then carry bit
    /// is set. If signed overflow occurs, then the overflow flag is set.
    ///
    /// Ex:
    ///
    /// 0x50 + 0x50 = 0xa0 | 80 + 80 = 160 | 80 + 80 = -96 <-- pos + pos = negative from
    /// signed overflow
    ///
    /// 0xd0 + 0x90 = 0x60 | 208 + 144 = 352 (- 256) = 96 | -48 + -112 <-- neg + neg = pos
    /// from signed overflow (plus sets the carry bit)
    ///
    /// The overflow flag can be set accordingly based on what (input carry ^ output carry)
    /// results to
    #[inline]
    fn adc(&mut self, value: u8) {
        let input_carry = (self.status & 0b0000_0001) == 0b0000_0001;
        let output_carry = (self.register_a > (0xFF - value))
            | ((self.register_a == (0xFF - value)) & input_carry);

        // sets the overflow flag
        if output_carry {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }

        self.update_overflow_flag(input_carry, output_carry);

        self.register_a = self
            .register_a
            .wrapping_add(value)
            .wrapping_add(if input_carry { 1 } else { 0 });

        self.update_zero_and_negative_flags(self.register_a);
    }

    /// INX - Increment X Register
    ///
    /// Increments the X register by 1 (wraps around on overflow) and sets
    /// the zero and negative flag as appropriate
    #[inline]
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// LDA - Load Accumulator
    ///
    /// Takes in an accumulator value and sets the zero and negative
    /// flag as appropriate
    #[inline]
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// TAX - Transfer of Accumulator to X
    ///
    /// Copies the content of the accumulator register into the X register
    /// and sets the zero and negative flag as appropriate
    #[inline]
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// Sets the zero and negative flag accordingly based on the result
    /// (coming from a register)
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html
    ///
    /// https://www.nesdev.org/wiki/Status_flags
    #[inline]
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010; // preserve prior status; set ZF to 1
        } else {
            self.status = self.status & 0b1111_1101; // preserve prior status; reset ZF to 0
        }

        if result & 0b1000_0000 == 0b1000_0000 {
            self.status = self.status | 0b1000_0000; // preserve prior status; set NF to 1
        } else {
            self.status = self.status & 0b0111_1111; // preserve prior status; reset NF to 0
        }
    }

    /// Sets the overflow flag accordingly in the status by doing a logical
    /// XOR on the input and output carry
    #[inline]
    fn update_overflow_flag(&mut self, input_carry: bool, output_carry: bool) {
        if input_carry != output_carry {
            self.status = self.status | 0b0100_0000;
        } else {
            self.status = self.status & 0b1011_1111;
        }
    }

    /// Runs in an infinite loop (until BRK) to do the following:
    /// - Fetch next exec instruction from instruction mem
    /// - Decode instruction
    /// - Exec instruction
    /// - Rinse and repeat
    pub fn interpret(&mut self, program: Vec<u8>) {
        // reset pc to 0 every time we call interpret on CPU
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            // all opsmode match to an instruction within the instruction
            // set described here: https://www.nesdev.org/obelisk-6502-guide/reference.html#
            match opscode {
                0x69 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.adc(param)
                }
                0xE8 => self.inx(),
                0xA9 => {
                    // For LDA (0xA9), it loads a byte of memory into the accumulator
                    // register (the param retrieves the byte of memory from program)
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(param);
                }
                0xAA => self.tax(),
                0x00 => return, // BRK instruction
                _ => todo!(),   // any other opscode that reaches here is essentially a no-op
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_adc_no_carry_out_and_no_overflow() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x50;
        cpu.interpret(vec![0x69, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x60);
        assert_eq!(cpu.status & 0b0000_0001, 0);
        assert_eq!(cpu.status & 0b0100_0000, 0);
    }

    #[test]
    fn test_adc_carry_out_and_overflow() {
        let mut cpu = CPU::new();
        cpu.register_a = 0xd0;
        cpu.interpret(vec![0x69, 0x90, 0x00]);

        assert_eq!(cpu.register_a, 0x60);
        assert_eq!(cpu.status & 0b0000_0001, 1);
        assert_eq!(cpu.status & 0b0100_0000, 0b0100_0000);
    }

    #[test]
    fn test_adc_carry_out_and_no_overflow() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x50;
        cpu.status = cpu.status | 0b1; // set carry in
        cpu.interpret(vec![0x69, 0xd0, 0x00]);

        assert_eq!(cpu.register_a, 0x21);
        assert_eq!(cpu.status & 0b0000_0001, 1);
        assert_eq!(cpu.status & 0b0100_0000, 0); // can't overflow if we're adding pos and neg
    }

    #[test]
    fn test_adc_no_carry_out_and_overflow() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x50;
        cpu.status = cpu.status | 0b1; // set carry in
        cpu.interpret(vec![0x69, 0x50, 0x00]);

        assert_eq!(cpu.register_a, 0xa1);
        assert_eq!(cpu.status & 0b0000_0001, 0);
        assert_eq!(cpu.status & 0b0100_0000, 0b0100_0000);
    }

    // TODO: Add cases to check if carry out and overflow
    // flag is set due to the existence of a carry in bit
    // (i.e. reg_a = 127 + mem_val = 128 + carry_bit = 1)

    #[test]
    fn test_carry_in_sets_carry_out() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x60;
        cpu.status = cpu.status | 0b1; // set carry in
        cpu.interpret(vec![0x69, 0x9f, 0x00]);

        assert_eq!(cpu.register_a, 0x0);
        assert_eq!(cpu.status & 0b0000_0001, 1);
        assert_eq!(cpu.status & 0b0100_0000, 0);
    }

    #[test]
    fn test_carry_in_sets_overflow() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x46;
        cpu.status = cpu.status | 0b1; // set carry in
        cpu.interpret(vec![0x69, 0x39, 0x00]);

        assert_eq!(cpu.register_a, 0x80);
        assert_eq!(cpu.status & 0b0000_0001, 0);
        assert_eq!(cpu.status & 0b0100_0000, 0b0100_0000);
    }
}
