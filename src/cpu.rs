pub struct CPU {
    pub register_a: u8, // accumulator CPU register (temp storage of data for calc)
    pub register_x: u8, // another register to store data
    pub register_y: u8, // another register to store data
    pub status: u8, // processor status register
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
            program_counter: 0
        }
    }

    /// LDA - Load Accumulator
    /// 
    /// Takes in an accumulator value and sets the zero and negative
    /// flag as appropriate
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// TAX - Transfer of Accumulator to X
    /// 
    /// Copies the content of the accumulator register into the X register
    /// and sets the zero and negative flag as appropriate
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// INX - Increment X Register
    /// 
    /// Increments the X register by 1 (wraps around on overflow) and sets
    /// the zero and negative flag as appropriate
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// sets the zero and negative flag accordingly from the instruction specs in
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010; // preserve prior status; set ZF to 1
        } else {
            self.status = self.status | 0b1111_1101; // preserve prior status; reset ZF to 0
        }

        if result & 0b1000_0000 == 0b1000_0000 {
            self.status = self.status | 0b1000_0000; // preserve prior status; set NF to 1
        } else {
            self.status = self.status & 0b0111_1111; // preserve prior status; reset NF to 0
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

            match opscode {
                0xA9 => {
                    // For LDA (0xA9), it loads a byte of memory into the accumulator
                    // register (the param retrieves the byte of memory from program)
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(param);
                },
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,
                _ => todo!() // any other opscode that reaches here is essentially a no-op
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
}
