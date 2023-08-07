#![allow(unused_macros)]
macro_rules! CUBE_VERT {
    () => (vec![
        V3::new(-0.5, -0.5, -0.5),
        V3::new( 0.5, -0.5, -0.5),
        V3::new( 0.5, -0.5,  0.5),
        V3::new(-0.5, -0.5,  0.5),
        V3::new(-0.5,  0.5, -0.5),
        V3::new( 0.5,  0.5, -0.5),
        V3::new( 0.5,  0.5,  0.5),
        V3::new(-0.5,  0.5,  0.5),
    ])
}
macro_rules! CUBE_EDGE {
    () => (vec![
        Edge::new(0, 1),
        Edge::new(1, 2),
        Edge::new(2, 3),
        Edge::new(3, 0),
        Edge::new(0, 4),
        Edge::new(1, 5),
        Edge::new(2, 6),
        Edge::new(3, 7),
        Edge::new(4, 5),
        Edge::new(5, 6),
        Edge::new(6, 7),
        Edge::new(7, 4),
    ])
}

macro_rules! TRI4_VERT {
    () => (vec![
        V3::new(-0.5, -0.5, -0.5),
        V3::new( 0.5, -0.5, -0.5),
        V3::new( 0.5, -0.5,  0.5),
        V3::new(-0.5, -0.5,  0.5),
        V3::new( 0.0,  0.5,  0.0),
    ])
}
macro_rules! TRI4_EDGE {
    () => (vec![
        Edge::new(0, 1),
        Edge::new(1, 2),
        Edge::new(2, 3),
        Edge::new(3, 0),
        Edge::new(0, 4),
        Edge::new(1, 4),
        Edge::new(2, 4),
        Edge::new(3, 4),
    ])
}

pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Edge {
    pub a: usize,
    pub b: usize,
}

pub struct State {
    // renderer
    pub v: Vec<V3>,
    pub e: Vec<Edge>,
    pub f: f32,
    pub r: f32,
    pub p: V3,

    // controls
    pub toggle_rotate: bool
}

impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> V3 {
        V3 { x, y, z }
    }
}

impl Edge {
    pub fn new(a: usize, b: usize) -> Edge {
        Edge { a, b }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            v: CUBE_VERT!(),
            e: CUBE_EDGE!(),
            f: 2.5,
            r: 0.0,
            p: V3::new(0.0, 0.0, 0.0),

            toggle_rotate: false,
        }
    }
}

pub fn fov_to_fl(fov: f32) -> f32 {
    2.0 / (2.0 * (fov / 2.0).tan())
}

pub fn render(state: &mut State, size: usize) -> Vec<Vec<u8>> {
    struct ProjectedPoint { x: f32, y: f32, d: f32 }
    struct ScreenPoint { x: isize, y: isize, d: f32 }

    let mut screen = vec![vec![0; size]; size];
    let mut verts = Vec::with_capacity(state.v.len());
    let sin = state.r.sin();
    let cos = state.r.cos();
    for v in state.v.iter() {
        let x = v.x * cos - v.z * sin - state.p.x;
        let y = v.y - state.p.y;
        let z = v.x * sin + v.z * cos - state.p.z;
        let d = z + state.f;
        verts.push(ProjectedPoint {
            x:  (x * state.f) / d,
            y: -(y * state.f) / d,
            d,
        });
    }
    let mut sverts = Vec::with_capacity(verts.len());
    for v in verts {
        sverts.push(ScreenPoint {
            x: (size as f32 * (v.x*0.5+0.5)) as isize,
            y: (size as f32 * (v.y*0.5+0.5)) as isize,
            d: v.d,
        });
    }
    for edge in state.e.iter() {
        let v1 = &sverts[edge.a];
        let v2 = &sverts[edge.b];

        if v1.d <= 0.0 || v2.d <= 0.0 {
            continue;
        }

        // Line drawing
        let mut x = v1.x;
        let mut y = v1.y;
        let mut dx = (v2.x-v1.x).abs();
        let mut dy = (v2.y-v1.y).abs();
        let s1 = (v2.x-v1.x).signum();
        let s2 = (v2.y-v1.y).signum();
        let interchange = if dy > dx {
            let t = dx;
            dx = dy;
            dy = t;
            true
        } else {
            false
        };
        let mut e = 2 * dy - dx;
        let a = 2 * dy;
        let b = 2 * dy - 2 * dx;
        plot(&mut screen, size, x, y, ((1.0-(v1.d / 10.0).min(0.5)) * 255.0) as u8);
        for i in 0..dx {
            if e < 0 {
                if interchange {
                    y += s2;
                } else {
                    x += s1;
                }
                e += a;
            } else {
                y += s2;
                x += s1;
                e += b;
            }
            let i = i as f32 / dx as f32;
            let l = v1.d * (1.0 - i) + v2.d * i;
            plot(&mut screen, size, x, y, ((1.0-(l / 10.0).min(0.5)) * 255.0) as u8);
        }
    }
    screen
}

fn plot(screen: &mut Vec<Vec<u8>>, size: usize, x: isize, y: isize, val: u8) {
    if  x >= 0 && x < size as isize &&
        y >= 0 && y < size as isize {
        if screen[y as usize][x as usize] < val {
            screen[y as usize][x as usize] = val
        }
    }
}
