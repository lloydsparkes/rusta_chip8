use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;

pub struct Display{
    canvas: Canvas<Window>,
}

impl Display{
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys.window(
            "rust-sdl2_gfx: draw line & FPSManager",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display { canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]){
        for row in 0..CHIP8_HEIGHT{
            for column in 0..CHIP8_WIDTH{
                let col = self.color(pixels[row][column]);
                self.canvas.set_draw_color(col);
                let c_scale = (column as i32) * SCALE_FACTOR as i32;
                let r_scale = (row as i32) * SCALE_FACTOR as i32;
                let _ = self.canvas.fill_rect(Rect::new(c_scale, r_scale, SCALE_FACTOR, SCALE_FACTOR));
            }
        }
        self.canvas.present();
    }

    fn color(&self, value: u8) -> pixels::Color {
        if value == 0 {
            pixels::Color::RGB(0, 0, 0)
        } else {
            pixels::Color::RGB(0, 250, 0)
        }
    }
}