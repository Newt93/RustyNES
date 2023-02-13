// CPU structure
pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,   // tracks current position in the program
    pub register_x: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,

        }
    }
}

/* Interpret method takes mutable reference to self 
because of need of modifying register_a */

/* CPU works in a constant cycle that:
Fetches instructions from instruction memory
Decodes the instruction
Exectutes the instruction
and then repeats the cycle. */

pub fn interpret(&mut self, program: Vec<u8>) {
    self.program_counter = 0;

    loop {
        let opscode = program[self.program_counter as usize];
        self.program_counter += 1;

        match opscode {
        // Implementation of LDA (0xA9) opcode using binary arithmetic.
        // You must to set or unset CPU flag depending on the results.
            0xa9 => {
                let param = program[self.program_counter as usize];
                self.program_counter += 1;
                self.register_a = param;

                if self.register_a == 0 {
                    self.status = self.status | 0b0000_0010;
                } else {
                    self.status = self.status & 0b1111_1101;
                }
                
                if self.register_a & 0b1000_0000 != 0 {
                    self.status = self.status | 0b1000_0000;
                } else {
                    self.status = self.status & 0b0111_1111;
                }
            }
            0x00 => {
                return;
            } 
            // implementing TAX
            0xAA => {
                self.register_x = self.register_a;

                if self.register_x == 0 {
                    self.status = self.status | 0b0000_0010;
                } else {
                    self.status = self.status & 0b1111_1101;
                }

                if self.register_x & 0b1000_0000 != 0 {
                    self.status = self.status | 0b1000_0000;
                } else {
                    self.status = self.status & 0b0111_1111;
                }
            }
        }
    }
}
