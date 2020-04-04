pub struct Chip8Cpu{
    memory: [u8; 4096],
    registers: [u8; 16],
    index_register: u16,
    program_counter: usize,
    graphics_memory: [bool; 64*32],
    delay_timer: u8,
    sound_timer: u8,

    stack: [usize; 16],
    stack_pointer: usize,
    input_keys: [u8; 16]
}

impl Chip8Cpu{
    pub fn new() -> Chip8Cpu {
        let new_cpu = Chip8Cpu{
            memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            graphics_memory: [false; 64*32],
            delay_timer: 0,
            sound_timer: 0,

            stack: [0; 16],
            stack_pointer: 0,
            input_keys: [0; 16]
        };
        return new_cpu;
    }

    pub fn cycle(&self) -> bool {
        // Fetch
        let opcode_1 = self.memory[self.program_counter];
        let opcode_2 = self.memory[self.program_counter+1];
        let joined_opcode = (opcode_1 as u16) << 8 | opcode_2 as u16;

        // Decode & Execute
        let op_index = joined_opcode & 0xF000;
        let op_addr = joined_opcode & 0x0FFF;
        let op_reg_a = ((joined_opcode >> 8) as u8) & 0x0F;
        let op_reg_b = (joined_opcode as u8) & 0xF0;
        let op_constant = joined_opcode as u8;

        match op_index {
            0x0000 => {
                match joined_opcode {
                    0x00E0 => { }, // Display Clear
                    0x00EE => { }, // Return
                    _ => { self.call(op_addr) }
                }
            }, 
            0x1000 => { self.jump(op_addr) },
            0x2000 => { self.call(op_addr) },
            0x3000 => { self.comp_constant(op_reg_a, op_constant, false) },
            0x4000 => { self.comp_constant(op_reg_a, op_constant, true) },
            0x5000 => { self.comp_register(op_reg_a, op_reg_b, false)},
            0x6000 => { self.set_or_add(op_reg_a, op_constant, false) },
            0x7000 => { self.set_or_add(op_reg_a, op_constant, true) },
            0x8000 => { 
                let lsb = op_constant & 0x0F;
                match lsb {
                    0x00 => {},
                    0x01 => {},
                    0x02 => {},
                    0x03 => {},
                    0x04 => {},
                    0x05 => {},
                    0x06 => {},
                    0x07 => {},
                    0x0E => {},
                    _ => {} // Error
                }                
            },
            0x9000 => { self.comp_register(op_reg_a, op_reg_b, true) },
            0xA000 => { self.set_index(op_addr)},
            0xB000 => { self.jump(self.get_reg16(0) + op_addr)},
            0xC000 => { self.rand_reg(op_reg_a, op_constant)},
            0xD000 => { self.draw_sprite(op_reg_a, op_reg_b, op_constant & 0x0F) },
            0xE000 => { 
                match op_constant {
                    0x9E => { self.comp_key(op_reg_a, true) },
                    0xA1 => { self.comp_key(op_reg_a, false) },
                    _ => {} // Error
                }
            },
            0xF000 => {
                match op_constant{
                    0x07 => { self.set_or_add(op_reg_a, self.delay_timer, false)},
                    0x0A => { self.set_or_add(op_reg_a, self.get_key(), false)},
                    0x15 => { self.set_timer(op_reg_a, &self.delay_timer)},
                    0x18 => { self.set_timer(op_reg_a, &self.sound_timer)},
                    0x1E => { self.update_index(op_reg_a, true)},
                    0x29 => { self.update_index_sprite(op_reg_a)},
                    0x33 => { self.set_bcd(op_reg_a)},
                    0x55 => { self.dump_reg(op_reg_a)},
                    0x65 => { self.load_reg(op_reg_a)},
                    _ => {} // Error
                }
             },
            _ => { } // Error
        }

        // Execute

        // Update Timers
        return false;
    }

    fn get_reg16(&self, reg: u8) -> u16 {
        return self.get_reg8(reg) as u16;
    }

    fn get_reg8(&self, reg: u8) -> u8 {
        return self.registers[reg as usize];
    }

    fn call(&self, dest: u16) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer = self.stack_pointer + 1;
        self.program_counter = dest as usize;
    }

    fn ret(&self) {
        self.stack_pointer = self.stack_pointer - 1;
        self.program_counter = self.stack[self.stack_pointer];
        self.program_counter = self.program_counter + 2;
    }

    fn jump(&self, dest: u16) {
        self.program_counter = dest as usize;
    }

    fn comp_constant(&self, source_reg: u8, constant: u8, not: bool){

    }

    fn comp_register(&self, source_reg: u8, constant: u8, not: bool){

    }

    fn set_or_add(&self, source_reg: u8, constant: u8, add: bool) {

    }

    pub fn should_draw(&self) -> bool {
        return false;
    }

    pub fn update_input(&self) {

    }
}

fn update_graphics(){

}

fn main() {
    println!("Hello, world!");

    // Setup Graphics
    // Setup Input

    let cpu = Chip8Cpu::new();

    let i = 0;
    while i == 0 {

        if !cpu.cycle() {
            break;
        }

        if cpu.should_draw(){
            update_graphics();
        }
        
        cpu.update_input();
    }
}
