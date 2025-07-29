use sdl3::Sdl;
use sdl3::video::{Window};
use sdl3::render::{Canvas};
use std::time::{Instant};

pub struct Engine {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    running: bool,
    last_frame_time: Instant,
    delta_time: f32,
}

impl Engine {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl_context: Sdl = sdl3::init().map_err(|e| e.to_string())?;
        let video_subsystem: sdl3::VideoSubsystem = sdl_context.video().map_err(|e| e.to_string())?;
        
        let window: Window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
            
        let canvas: Canvas<Window> = window.into_canvas();
            
        Ok(Engine {
            sdl_context,
            canvas,
            running: false,
            last_frame_time: Instant::now(),
            delta_time: 0.0,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.running = true;
        while self.running {
            let current_time: Instant = Instant::now();
            self.delta_time = (current_time - self.last_frame_time).as_secs_f32();
            self.last_frame_time = current_time;

            self.canvas.set_draw_color(sdl3::pixels::Color::RGB(15, 0, 30));
            self.canvas.clear();
            self.canvas.present();

            std::thread::sleep(std::time::Duration::from_millis(16));
        }
        Ok(())
    }
}

pub fn hello_engine() {
    println!("Hello from the engine!");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
