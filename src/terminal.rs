use crossterm::{*, style::*, event::*};
use std::io::*;
use std::time::*;
use std::ptr::null_mut;

pub static mut LOGS: Vec<(SystemTime, String)> = Vec::new();

pub static mut STDOUT_BUF: *mut BufWriter<Stdout> = null_mut();

pub fn push_log(s: &str) {
    unsafe {
        let t = SystemTime::now();
        LOGS.push((t, s.to_string()));
    }
}

pub fn init(so: &mut BufWriter<Stdout>) -> std::result::Result<u16, Box<dyn std::error::Error>> {
    unsafe { STDOUT_BUF = so as *mut BufWriter<Stdout> };

    terminal::enable_raw_mode()?;
    execute!(stdout(),
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide
    )?;

    let (c, r) = terminal::size()?;

    Ok(c.min(r << 1))
}

pub fn prep_exit() -> core::result::Result<(), Box<dyn std::error::Error>> {
    execute!(stdout(),
        terminal::LeaveAlternateScreen,
        cursor::Show
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn push_image(image: Vec<Vec<bool>>, msg: &str) -> core::result::Result<(), Box<dyn std::error::Error>> {
    let so = unsafe { &mut *STDOUT_BUF };
    queue!(so, cursor::MoveTo(0, 0))?;
    for y in image.chunks(2) {
        let rle = rle_row(y);
        for i in rle.into_iter() {
            let mut c = "\u{2580}".repeat(i.n).stylize();
            c = if i.d.0 {
                c.with(Color::White)
            } else {
                c.with(Color::DarkGrey)
            };
            c = if i.d.1 {
                c.on(Color::White)
            } else {
                c.on(Color::DarkGrey)
            };
            queue!(so,
                style::PrintStyledContent(c)
            )?;
        }

        queue!(so, cursor::MoveToNextLine(1))?;
    }

    queue!(so,
        terminal::Clear(terminal::ClearType::CurrentLine),
        style::PrintStyledContent(
            msg .with(Color::White)
                .on  (Color::Black)
        )
    )?;

    so.flush()?;

    Ok(())
}

struct RLEChunk {
    n: usize,
    d: (bool, bool)
}
fn rle_row(src: &[Vec<bool>]) -> Vec<RLEChunk> {
    let mut ic_at = 0;
    let mut ichunks = Vec::with_capacity(src[0].len());
    let mut fchunks = Vec::new();
    for i in 0..src[0].len() {
        if src.len() == 2 {
            ichunks.push(RLEChunk {
                n: 1,
                d: (src[0][i], src[1][i])
            })
        } else {
            ichunks.push(RLEChunk {
                n: 1,
                d: (src[0][i], false)
            })
        }
    }
    let mut n = 0;
    let mut d = ichunks[0].d;
    while let Some(this) = next(&ichunks, &mut ic_at) {
        if this.d == d {
            n += 1;
        } else {
            fchunks.push(RLEChunk {
                n, d
            });
            n = 1;
            d = this.d;
        }
    }
    fchunks.push(RLEChunk {
        n, d
    });
    fchunks
}

fn next<'a, A>(a: &'a Vec<A>, i: &'a mut usize) -> Option<&'a A> {
    let b = a.get(*i);
    *i += 1;
    b
}

pub fn show(s: &str) -> core::result::Result<(), Box<dyn std::error::Error>> {
    let so = unsafe { &mut *STDOUT_BUF };
    execute!(so,
        cursor::MoveTo(0, 0),
        style::PrintStyledContent(s
            .with(Color::White)
            .on  (Color::DarkGrey)
        )
    )?;
    Ok(())
}

pub fn handle_input(el: Duration, state: &mut crate::renderer::State) -> core::result::Result<(), Box<dyn std::error::Error>> {
    let pr = poll(
        Duration::from_millis(
            ((1000.0 / crate::MAX_FPS) as u128).checked_sub(el.as_millis()).unwrap_or(0) as u64
        )
    )?;
    if pr {
        match read()? {
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
                unsafe {
                    let mut s = String::new();

                    for i in &LOGS {
                        s += &format!("{:.02}s ago: {}\n", i.0.elapsed()?.as_secs_f64(), i.1);
                    }

                    std::fs::write("logs.txt", s)?;
                }

                prep_exit()?;
                std::process::exit(0);
            },
            Event::Key(KeyEvent { code: KeyCode::Char('r'), kind: KeyEventKind::Press, .. }) => {
                state.toggle_rotate ^= true;
            },
            _ => ()
        }
    }

    Ok(())
}
