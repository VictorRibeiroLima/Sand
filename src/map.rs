use sdl2::{render::Canvas, video::Window};

use crate::{sand::Sand, SIZE};

pub struct Map {
    map: [[Option<Sand>; SIZE]; SIZE],
}

impl Map {
    pub fn new() -> Map {
        let map = Map {
            map: [[None; SIZE]; SIZE],
        };
        return map;
    }

    pub fn set(&mut self, x: usize, y: usize, sand: Sand) {
        if let None = self.map[y][x] {
            self.map[y][x] = Some(sand);
        }
    }

    pub fn apply_gravity(&mut self) {
        for y in (0..SIZE).rev() {
            for x in 0..SIZE {
                if let Some(sand) = self.map[y][x] {
                    if y < SIZE - 1 {
                        match self.map[y + 1][x] {
                            None => {
                                self.map[y][x] = None;
                                self.map[y + 1][x] = Some(sand);
                            }
                            _ => {
                                let left = x > 0;
                                let right = x < SIZE - 1;
                                let new_y = y + 1;
                                if new_y >= SIZE {
                                    continue;
                                }
                                let mut side = None;

                                if left && right {
                                    let right_x = x + 1;
                                    let left_x = x - 1;
                                    let right_sand = self.map[new_y][right_x];
                                    let left_sand = self.map[new_y][left_x];
                                    side = match (left_sand, right_sand) {
                                        (None, None) => Some(random_side(left_x, right_x)),
                                        (Some(_), None) => Some(right_x),
                                        (None, Some(_)) => Some(left_x),
                                        (Some(_), Some(_)) => None,
                                    };
                                } else if left {
                                    if let None = self.map[new_y][x - 1] {
                                        side = Some(x - 1);
                                    }
                                } else if right {
                                    if let None = self.map[new_y][x + 1] {
                                        side = Some(x + 1);
                                    }
                                }
                                if let Some(side) = side {
                                    self.map[y][x] = None;
                                    self.map[new_y][side] = Some(sand);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                if let Some(sand) = self.map[y][x] {
                    /*
                    To draw each pixel,one by one its very slow after some time
                    later we need to draw a line of pixels instead of one by one
                     */
                    let color = sand.color;
                    canvas.set_draw_color(color);
                    canvas.draw_point((x as i32, y as i32)).unwrap();
                }
            }
        }
    }
}

fn random_side(left_x: usize, right_x: usize) -> usize {
    if rand::random() {
        return left_x;
    } else {
        return right_x;
    }
}
