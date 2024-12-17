use nannou::prelude::*;

pub struct Wall {
    pub start_point: Vec2,
    pub end_point: Vec2,
}

impl Wall {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self {
            start_point: start,
            end_point: end,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.line()
            .start(self.start_point)
            .color(WHITE)
            .weight(1.5)
            .end(self.end_point);
    }
}
