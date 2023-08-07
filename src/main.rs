#![allow(cast_ref_to_mut)]

use std::time::SystemTime;
use std::io::*;

pub const STDOUT_BUF_SIZE: usize = 96*KB;
pub const COMPRESSION_DIFF: u8 = 16;
pub const MAX_FPS: f64 = 60.0;
pub const FOV: f32 = 70.0;

const MB: usize = KB * 1024;
const KB: usize = 1024;

mod terminal;
mod renderer;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut so = BufWriter::with_capacity(STDOUT_BUF_SIZE, stdout());
    let mut size = terminal::init(&mut so)? as usize;

    let mut fps = 0.0;
    let mut out_size = 0;

    let mut state = renderer::State::default();
    state.f = renderer::fov_to_fl(FOV);
    
    loop {
        let s = SystemTime::now();

        let rr = renderer::render(&mut state, size);

        out_size = terminal::push_image(rr, &format!("FPS {fps:.1} total / {:.1} render\r\nBuffer size {:.1}KB / {:.1}KB", 1000.0 / (s.elapsed()?.as_nanos() as f64 / 1e+6), out_size as f32 / KB as f32, STDOUT_BUF_SIZE as f32 / KB as f32))?;
        size = terminal::handle_input(s.elapsed()?, &mut state)?.unwrap_or(size);

        let total = s.elapsed()?.as_nanos() as f64 / 1e+6;
        fps = 1000.0 / total;

        if state.toggle_rotate {
            state.r += (std::f32::consts::TAU * 0.25) * (total as f32 / 1000.0);
        }
    }
}
