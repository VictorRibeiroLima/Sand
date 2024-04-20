extern crate sdl2;

use map::Map;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
mod map;
mod sand;

const SIZE: usize = 900;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("sand", SIZE as u32, SIZE as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut map = Map::new();

    let mut mouse_down = false;
    let mut color = gen_random_color();
    let black = Color::RGB(0, 0, 0);
    let mut time_freeze = false;

    'running: loop {
        canvas.set_draw_color(black);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { .. } => {
                    color = gen_random_color();
                    mouse_down = true;
                }
                Event::MouseButtonUp { .. } => {
                    mouse_down = false;
                }
                Event::MouseMotion { x, y, .. } => {
                    if mouse_down {
                        let punch = gen_punch(x, y);
                        for (x, y) in punch {
                            if x >= SIZE || y >= SIZE {
                                continue;
                            }
                            map.set(x, y, sand::Sand::new(color));
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    time_freeze = !time_freeze;
                }
                _ => {}
            }
        }
        if !time_freeze {
            map.apply_gravity();
        }
        map.draw(&mut canvas);
        canvas.present();
    }
}

fn gen_random_color() -> Color {
    let mut r = rand::random::<u8>();
    let mut g = rand::random::<u8>();
    let mut b = rand::random::<u8>();
    if r < 50 {
        let offset = 255 - r;
        r += offset;
    }
    if g < 50 {
        let offset = 255 - g;
        g += offset;
    }
    if b < 50 {
        let offset = 255 - b;
        b += offset;
    }
    Color::RGB(r, g, b)
}

/*
    the idea is to create a punch of sand from a specific point

    n n n
    n x n
    n n n

    where x is the origin point and n is the surrounding points

*/
fn gen_punch(x_origin: i32, y_origin: i32) -> Vec<(usize, usize)> {
    let mut punch = Vec::new();
    for y in y_origin - 1..y_origin + 2 {
        for x in x_origin - 1..x_origin + 2 {
            if x >= 0 && y >= 0 {
                punch.push((x as usize, y as usize));
            }
        }
    }
    punch
}
