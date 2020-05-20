extern crate sdl2;

mod ui;
mod chip8;

use std::env;
use std::thread;
use std::time::Duration;
use std::io;
use std::io::prelude::*;

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
            dsp.draw(cpu.gfx());
            pause();
        }
    
        if should_beep {
            // Start beep
        } else {
            // End Beep
        }

        thread::sleep(sleep_duration);
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
    println!();
}
