extern crate sdl2;

mod ui;
mod chip8;

use std::env;
use std::thread;
use std::time::Duration;

use ui::Display;
use ui::Input;
use chip8::Chip8Cpu;

fn main() {
    let sleep_duration = Duration::from_millis(2);
    let sdl_context = sdl2::init().unwrap();
    let args: Vec<String> = env::args().collect();
    let cartridge_filename = &args[1];

    // Setup Graphics
    let mut dsp = Display::new(&sdl_context);
    // Setup Input
    let mut input = Input::new(&sdl_context);

    let mut cpu = Chip8Cpu::new();
    cpu.load_rom(cartridge_filename);

    while let Ok(keys) = input.poll() {

        let (should_draw, should_beep) = cpu.cycle(keys);

        if should_draw {
            dsp.draw(&cpu.graphics_memory);
        }
    
        if(should_beep){
            // Start beep
        } else {
            // End Beep
        }

        thread::sleep(sleep_duration);
    }
}
