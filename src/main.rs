extern crate sdl2;

mod ui;
mod chip8;

use std::env;
use std::thread;
use std::time::Duration;

use ui::Display;
use chip8::Chip8Cpu;

fn main() {
    let sleep_duration = Duration::from_millis(2);
    let sdl_context = sdl2::init().unwrap();
    let args: Vec<String> = env::args().collect();
    let cartridge_filename = &args[1];

    // Setup Graphics
    let mut dsp = Display::new(&sdl_context);
    // Setup Input

    let mut cpu = Chip8Cpu::new();

    let i = 0;
    while i == 0 {

        if !cpu.cycle() {
            break;
        }

        if cpu.should_draw(){
            dsp.draw(&cpu.graphics_memory);
        }
        
        cpu.update_input();

        thread::sleep(sleep_duration);
    }
}
