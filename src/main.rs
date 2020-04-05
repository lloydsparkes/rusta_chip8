use std::u8;
use rand::Rng;

pub struct Chip8Cpu{
    memory: [u8; 4096],
    registers: [u8; 16],
    index_register: u16,
    program_counter: usize,
    graphics_memory: [bool; 64*32],
    delay_timer: u8,
    sound_timer: u8,
    next_increment: usize,

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
            next_increment: 2,

            stack: [0; 16],
            stack_pointer: 0,
            input_keys: [0; 16]
        };
        return new_cpu;
    }

    pub fn cycle(&mut self) -> bool {
        // Fetch
        let opcode_1 = self.memory[self.program_counter];
        let opcode_2 = self.memory[self.program_counter+1];
        let joined_opcode = (opcode_1 as u16) << 8 | opcode_2 as u16;

        // Decode & Execute
        let op_i = joined_opcode & 0xF000; //Op Code Index
        let op_ad = joined_opcode & 0x0FFF; //Op Code Address
        let op_ra = ((joined_opcode >> 8) as u8) & 0x0F; //Op Code Reg A
        let op_rb = (joined_opcode as u8) & 0xF0; //Op Code Reg B
        let op_c = joined_opcode as u8; // Op Code Constant

        self.next_increment = 2;

        match op_i {
            0x0000 => {
                match joined_opcode {
                    0x00E0 => { }, // Display Clear
                    0x00EE => { }, // Return
                    _ => { self.call(op_ad) }
                }
            }, 
            0x1000 => { self.jump(op_ad) },
            0x2000 => { self.call(op_ad) },
            0x3000 => { self.compare(self.g_r8(op_ra), op_c, false) },
            0x4000 => { self.compare(self.g_r8(op_ra), op_c, true) },
            0x5000 => { self.compare(self.g_r8(op_ra), self.g_r8(op_rb), false)},
            0x6000 => { self.s_r(op_ra, op_c) },
            0x7000 => { self.s_r(op_ra, self.g_r8(op_ra) + op_c) },
            0x8000 => { 
                let lsb = op_c & 0x0F;
                match lsb {
                    0x00 => {self.s_r(op_ra, self.g_r8(op_rb))},
                    0x01 => {self.s_r(op_ra, self.g_r8(op_ra)|self.g_r8(op_rb))},
                    0x02 => {self.s_r(op_ra, self.g_r8(op_ra)&self.g_r8(op_rb))},
                    0x03 => {self.s_r(op_ra, self.g_r8(op_ra)^self.g_r8(op_rb))},
                    0x04 => {self.add(op_ra, self.g_r8(op_ra),self.g_r8(op_rb))},
                    0x05 => {self.sub(op_ra, self.g_r8(op_ra),self.g_r8(op_rb))},
                    0x06 => {self.shift(op_ra, self.g_r8(op_ra), false)},
                    0x07 => {self.sub(op_ra, self.g_r8(op_rb),self.g_r8(op_ra))},
                    0x0E => {self.shift(op_ra, self.g_r8(op_ra), true)},
                    _ => {} // Error
                }                
            },
            0x9000 => { self.compare(op_ra, op_rb, true) },
            0xA000 => { self.s_i(op_ad)},
            0xB000 => { self.jump(self.g_r16(0) + op_ad)},
            0xC000 => { self.s_r(op_ra, self.rand(op_c))},
            0xD000 => { self.draw_sprite(self.g_r8(op_ra), self.g_r8(op_rb)) },
            0xE000 => { 
                match op_c {
                    0x9E => { self.compare(self.g_r8(op_ra), self.get_key(), true) },
                    0xA1 => { self.compare(self.g_r8(op_ra), self.get_key(), false) },
                    _ => {} // Error
                }
            },
            0xF000 => {
                match op_c{
                    0x07 => { self.s_r(op_ra, self.delay_timer)},
                    0x0A => { self.s_r(op_ra, self.get_key())},
                    0x15 => { self.s_dt(self.g_r8(op_ra))},
                    0x18 => { self.s_st(self.g_r8(op_ra))},
                    0x1E => { self.s_i(self.index_register + self.g_r16(op_ra))},
                    0x29 => { self.s_i(self.get_sprite(self.g_r8(op_ra)))},
                    0x33 => { self.set_bcd(self.g_r8(op_ra))},
                    0x55 => { self.dump_reg(op_ra)},
                    0x65 => { self.load_reg(op_ra)},
                    _ => {} // Error
                }
             },
            _ => { } // Error
        }
        self.program_counter = self.program_counter + self.next_increment;

        // Update Timers
        return false;
    }

    fn draw_sprite(&self, x: u8, y: u8){

    }

    fn rand(&self, constant: u8) -> u8{
        let mut rng = rand::thread_rng();

        let rd: u8 = rng.gen();
        return rd & constant;
    }

    fn dump_reg(&self, limit: u8) {

    }

    fn load_reg(&self, limit: u8) {
        
    }

    fn get_key(&self, ) -> u8 {
        return 0;
    }

    fn get_sprite(&self, sprite_n: u8) -> u16 {
        return 0;
    }

    fn set_bcd(&self, value: u8){

    }

    fn add(&mut self, out_r: u8, a: u8, b: u8) {
        let (res, carry) = a.overflowing_add(b);
        if carry {
            self.s_r(0x0F, 0x01)
        } else {
            self.s_r(0x0F, 0x00)
        }
        self.s_r(out_r, res);
    }

    fn sub(&mut self, out_r: u8, a: u8, b: u8) {
        if a > b {
            self.s_r(0x0F, 0x01)
        } else {
            self.s_r(0x0F, 0x00)
        }
        self.s_r(out_r, a - b);
    }

    fn shift(&mut self, out_r: u8, a: u8, left: bool) {
        if left {
            if a & 0b1000_0000 == 0b1000_0000 {
                self.s_r(0x0F, 0x01);
            } else {
                self.s_r(0x0F, 0x00)
            }
            self.s_r(out_r, a << 1);
        } else {
            if a & 0b0000_0001 == 0b0000_0001 {
                self.s_r(0x0F, 0x01);
            } else {
                self.s_r(0x0F, 0x00)
            }
            self.s_r(out_r, a >> 1);
        }
    }

    fn g_r16(&self, reg: u8) -> u16 {
        return self.g_r8(reg) as u16;
    }

    fn g_r8(&self, reg: u8) -> u8 {
        return self.registers[reg as usize];
    }

    fn s_i(&mut self, value: u16){
        self.index_register = value;
    }

    fn s_st(&mut self,  value: u8){
        self.sound_timer = value;
    }

    fn s_dt(&mut self,  value: u8){
        self.delay_timer = value;
    }

    fn call(&mut self, dest: u16) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer = self.stack_pointer + 1;
        self.program_counter = dest as usize;
        self.next_increment = 0;
    }

    fn ret(&mut self) {
        self.stack_pointer = self.stack_pointer - 1;
        self.program_counter = self.stack[self.stack_pointer];
    }

    fn jump(&mut self, dest: u16) {
        self.program_counter = dest as usize;
        self.next_increment = 0;
    }

    fn compare(&mut self, left: u8, right: u8, not: bool){
        let comp_result = left == right;
        if (!not && comp_result) || (not && !comp_result) {
             self.next_increment = 4;
             return;
        }
    }

    fn s_r(&mut self, dest_reg: u8, value: u8) {
        self.registers[dest_reg as usize] = value;
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

    let mut cpu = Chip8Cpu::new();

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
