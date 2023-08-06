#![allow(cast_ref_to_mut)]

use std::time::SystemTime;
use std::io::*;

pub const STDOUT_BUF_SIZE: usize = 16*MB;
pub const MAX_FPS: f64 = 60.0;

const MB: usize = KB * 1024;
const KB: usize = 1024;

mod terminal;
mod renderer;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut so = BufWriter::with_capacity(STDOUT_BUF_SIZE, stdout());
    let size = terminal::init(&mut so)?;
    let mut fps = 0.0;

    let mut state = renderer::State::default();
    
    loop {
        let s = SystemTime::now();

        let rr = renderer::render(&mut state, size as usize);

        terminal::push_image(rr, &format!("Total FPS {fps:.1} Render FPS {:.1}", 1000.0 / s.elapsed()?.as_millis() as f64))?;
        terminal::handle_input(s.elapsed()?, &mut state)?;

        let total = s.elapsed()?.as_nanos() as f64 / 1e+6;
        fps = 1000.0 / total;

        if state.toggle_rotate {
            state.r += (std::f32::consts::TAU * 0.25) * (total as f32 / 1000.0);
        }
    }
}
