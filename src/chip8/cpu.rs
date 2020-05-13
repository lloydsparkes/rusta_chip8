#[path = "../ui/display.rs"] mod ui;

use rand::Rng;
use std::u8;
use std::fs::File;
use std::io::Read;

use ui::{CHIP8_WIDTH, CHIP8_HEIGHT};

pub struct Chip8Cpu{
    memory: [u8; 4096],
    registers: [u8; 16],
    index_register: u16,
    program_counter: usize,
    pub graphics_memory: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    delay_timer: u8,
    sound_timer: u8,
    next_increment: usize,
    refresh_graphics: bool,

    stack: [usize; 16],
    stack_pointer: usize,
    pub input_keys: [u8; 16]
}

impl Chip8Cpu{
    pub fn new() -> Chip8Cpu {
        let new_cpu = Chip8Cpu{
            memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            graphics_memory: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            next_increment: 2,
            refresh_graphics: false,

            stack: [0; 16],
            stack_pointer: 0,
            input_keys: [0; 16]
        };
        return new_cpu;
    }

    pub fn load_rom(&mut self, filename: &str) {
        let mut f = File::open(filename).expect("file not found");
        let mut buffer = [0u8; 3584];
        let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
            bytes_read
        } else {
            0
        };

        for (i, &byte) in FONT_SET.iter().enumerate() {
            self.memory[i] = byte;
        }

        for (i, &byte) in buffer.iter().enumerate() {
            let addr = 0x200+i;
            if addr < 4096 {
                self.memory[0x200+i] = byte;
            } else {
                break;
            }
        }
    }

    pub fn cycle(&mut self, keypad: [bool; 16]) -> (bool, bool) {
        // Fetch
        let opcode_1 = self.memory[self.program_counter];
        let opcode_2 = self.memory[self.program_counter+1];
        let joined_opcode = (opcode_1 as u16) << 8 | opcode_2 as u16;

        // Decode & Execute
        let op_i = joined_opcode & 0xF000; //Op Code Index
        let op_ad = joined_opcode & 0x0FFF; //Op Code Address
        let op_ra = ((joined_opcode >> 8) as u8) & 0x0F; //Op Code Reg A
        let op_rb = ((joined_opcode as u8) & 0xF0) >> 4; //Op Code Reg B
        let op_c = joined_opcode as u8; // Op Code Constant

        self.next_increment = 2;

        println!("PC: {:X} OP {:X} OP_I: {:X} OP_ADDR: {:X} OP_REGA: {:X} OP_REGB: {:X} OP_CONSTANT:{:X}", self.program_counter, joined_opcode, op_i, op_ad, op_ra, op_rb, op_c);
        match op_i {
            0x0000 => {
                match joined_opcode {
                    0x00E0 => { }, // Display Clear
                    0x00EE => { self.ret() }, // Return
                    _ => { } //self.call(op_ad) }
                }
            }, 
            0x1000 => { self.jump(op_ad) },
            0x2000 => { self.call(op_ad) },
            0x3000 => { self.compare(self.g_r8(op_ra), op_c, false) },
            0x4000 => { self.compare(self.g_r8(op_ra), op_c, true) },
            0x5000 => { self.compare(self.g_r8(op_ra), self.g_r8(op_rb), false)},
            0x6000 => { self.s_r(op_ra, op_c) },
            0x7000 => { self.s_r(op_ra, ((self.g_r8(op_ra) as u16) + op_c as u16) as u8) },
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
            0xD000 => { self.draw_sprite(op_ra, op_rb, op_ad as usize) },
            0xE000 => { 
                match op_c {
                    0x9E => { self.compare(self.g_r8(op_ra), self.get_key(keypad), true) },
                    0xA1 => { self.compare(self.g_r8(op_ra), self.get_key(keypad), false) },
                    _ => {} // Error
                }
            },
            0xF000 => {
                match op_c{
                    0x07 => { self.s_r(op_ra, self.delay_timer)},
                    0x0A => {
                        if self.get_key(keypad) > 0 { 
                            self.s_r(op_ra, self.get_key(keypad))
                        } else {
                            // Wait for input, so dont increment the increment
                            self.next_increment = 0;
                        }
                    },
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
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        return (self.refresh_graphics, self.sound_timer > 0);
    }

    fn get_key(&self, keypad: [bool; 16]) -> u8 {
        for i in 0..keypad.len(){
            if keypad[i] {
                return i as u8;
            }
        }
        return 0;
    }

    fn draw_sprite(&mut self, x: u8, y: u8, n: usize){
        let base_src = self.index_register as usize;
        let source_x = self.g_r8(x) as usize;
        let source_y = self.g_r8(y) as usize;
        let mut flag = 0;
        for byte in 0usize..n {
            let y = (source_y + byte) % CHIP8_HEIGHT;
            for bit in 0usize..8{
                let x = source_x + bit % CHIP8_WIDTH;
                let color = (self.memory[base_src+byte] >> (7-bit)) & 1;
                flag |= color & self.graphics_memory[y][x];
                self.graphics_memory[y][x] = color;
            }
        }
        self.s_r(0x0F, flag);
        self.refresh_graphics = true;
    }

    fn rand(&self, constant: u8) -> u8{
        let mut rng = rand::thread_rng();

        let rd: u8 = rng.gen();
        return rd & constant;
    }

    fn dump_reg(&mut self, limit: u8) {
        let max_var = limit as usize;
        let mut dest = self.index_register as usize;
        for n in 0..max_var {
            self.memory[dest] = self.registers[n];
            dest = dest + 1;
        }
    }

    fn load_reg(&mut self, limit: u8) {
        let max_var = limit as usize;
        let mut src = self.index_register as usize;
        for n in 0..max_var {
            self.registers[n] = self.memory[src];
            src = src + 1;
        }
    }

    fn get_sprite(&self, sprite_n: u8) -> u16 {
        return 0;
    }

    fn set_bcd(&mut self, value: u8){
        let dest = self.index_register as usize;
        let (s_value, _) = value.overflowing_shr(8);
        self.memory[dest] = s_value / 100;
        self.memory[dest + 1] = (s_value / 10) % 10;
        self.memory[dest + 2] = (s_value % 100) % 10;
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
        let (res, carry) = a.overflowing_sub(b);
        if carry {
            self.s_r(0x0F, 0x01)
        } else {
            self.s_r(0x0F, 0x00)
        }
        self.s_r(out_r, res);
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
        println!("G_R8 {}", reg);
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
        println!("CALL SP:{:X} DEST:{:X} PC:{:X}", self.stack_pointer, dest, self.program_counter);
        self.stack[self.stack_pointer as usize] = self.program_counter + 2;
        self.stack_pointer += 1;
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
}

pub const FONT_SET: [u8; 80] = [
    0xF0,
    0x90,
    0x90,
    0x90,
    0xF0,
    0x20,
    0x60,
    0x20,
    0x20,
    0x70,
    0xF0,
    0x10,
    0xF0,
    0x80,
    0xF0,
    0xF0,
    0x10,
    0xF0,
    0x10,
    0xF0,
    0x90,
    0x90,
    0xF0,
    0x10,
    0x10,
    0xF0,
    0x80,
    0xF0,
    0x10,
    0xF0,
    0xF0,
    0x80,
    0xF0,
    0x90,
    0xF0,
    0xF0,
    0x10,
    0x20,
    0x40,
    0x40,
    0xF0,
    0x90,
    0xF0,
    0x90,
    0xF0,
    0xF0,
    0x90,
    0xF0,
    0x10,
    0xF0,
    0xF0,
    0x90,
    0xF0,
    0x90,
    0x90,
    0xE0,
    0x90,
    0xE0,
    0x90,
    0xE0,
    0xF0,
    0x80,
    0x80,
    0x80,
    0xF0,
    0xE0,
    0x90,
    0x90,
    0x90,
    0xE0,
    0xF0,
    0x80,
    0xF0,
    0x80,
    0xF0,
    0xF0,
    0x80,
    0xF0,
    0x80,
    0x80,
];
