pub struct CPU {
    pub registers: [i32; 8],
    pub memory: [i32; 1024],
    pub pc: usize,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: [0; 8],
            memory: [0; 1024],
            pc: 0,
        }
    }

    pub fn optimized_arithmetic_operations(&mut self, op_code: u8, reg1: usize, reg2: usize, reg3: usize) {
        match op_code {
            0 => self.registers[reg1] = self.registers[reg2].wrapping_add(self.registers[reg3]), // ADD
            1 => self.registers[reg1] = self.registers[reg2].wrapping_sub(self.registers[reg3]), // SUB
            2 => self.registers[reg1] = self.registers[reg2].wrapping_mul(self.registers[reg3]), // MUL
            3 => self.registers[reg1] = self.registers[reg2].wrapping_div(self.registers[reg3]), // DIV
            _ => panic!("Unknown arithmetic operation"),
        }
    }

    pub fn optimized_control_flow(&mut self, op_code: u8, reg1: usize, reg2: usize, address: usize) {
        match op_code {
            0 => if self.registers[reg1] == self.registers[reg2] { self.pc = address; }, // BEQ
            1 => if self.registers[reg1] != self.registers[reg2] { self.pc = address; }, // BNE
            2 => if self.registers[reg1] < self.registers[reg2] { self.pc = address; },  // BLT
            3 => if self.registers[reg1] > self.registers[reg2] { self.pc = address; },  // BGT
            _ => panic!("Unknown control flow operation"),
        }
    }

    pub fn optimized_memory_handling(&mut self, op_code: u8, reg1: usize, address: usize) {
        match op_code {
            0 => self.registers[reg1] = self.memory[address], // LOAD
            1 => self.memory[address] = self.registers[reg1], // STORE
            _ => panic!("Unknown memory handling operation"),
        }
    }
}