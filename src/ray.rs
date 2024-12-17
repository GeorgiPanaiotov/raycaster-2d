use nannou::prelude::*;

pub struct Ray {
    pub pos: Vec2,
    pub dir: Vec2,
}

impl Ray {
    pub fn new(pos: Vec2, dir: Vec2) -> Self {
        Self { pos, dir }
    }

    pub fn check_collision(&self, pt2: Vec2, dir2: Vec2) -> Option<Vec2> {
        let den = (pt2.x - dir2.x) * (self.pos.y - self.pos.y + self.dir.y)
            - (pt2.y - dir2.y) * (self.pos.x - self.pos.x + self.dir.x);

        if den.abs() == 0.0 {
            return None;
        }

        let t = ((pt2.x - self.pos.x) * (self.pos.y - self.pos.y + self.dir.y)
            - (pt2.y - self.pos.y) * (self.pos.x - self.pos.x + self.dir.x))
            / den;

        let u = ((pt2.x - dir2.x) * (pt2.y - self.pos.y) - (pt2.y - dir2.y) * (pt2.x - self.pos.x))
            / den;

        if t > 0.0 && t < 1.0 && u > 0.0 {
            let x = pt2.x + t * (dir2.x - pt2.x);
            let y = pt2.y + t * (dir2.y - pt2.y);
            Some(vec2(x, y))
        } else {
            return None;
        }
    }

    pub fn update(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn draw(&self, draw: &Draw, end: Vec2) {
        draw.line()
            .start(self.pos)
            .end(end)
            .stroke_weight(1.5)
            .color(WHITE);
    }
}
