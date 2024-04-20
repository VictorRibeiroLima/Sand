use sdl2::pixels::Color;

#[derive(Clone, Copy, Debug)]
pub struct Sand {
    pub color: Color,
}

impl Sand {
    pub fn new(color: Color) -> Sand {
        Sand { color: color }
    }
}
