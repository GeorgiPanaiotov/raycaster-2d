use core::f32;

use nannou::prelude::*;

use crate::{ray::Ray, wall::Wall};

pub struct Player {
    pub pos: Vec2,
    pub rays: Vec<Ray>,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        let mut a = 0;
        let mut rays: Vec<Ray> = vec![];

        while a <= 360 {
            a += 5;
            let radians = a.to_f32().unwrap().to_radians();
            let ray = Ray::new(pos, vec2(radians.cos(), radians.sin()));
            rays.push(ray);
        }

        Self { pos, rays }
    }

    pub fn update(&mut self, pos: Vec2) {
        self.pos = pos;

        for ray in &mut self.rays {
            ray.update(pos);
        }
    }

    pub fn draw(&self, draw: &Draw, walls: &Vec<Wall>) {
        draw.ellipse()
            .x_y(self.pos.x, self.pos.y)
            .color(WHITE)
            .w_h(5.0, 5.0);

        for ray in &self.rays {
            let mut closest_wall: Option<Vec2> = None;
            let mut record = f32::MAX;

            for wall in walls {
                let end_point = ray.check_collision(wall.start_point, wall.end_point);
                if end_point.is_some() {
                    let distance = end_point.unwrap().distance(ray.pos);
                    if distance < record {
                        record = distance;
                        closest_wall = end_point;
                    }
                }
            }
            if closest_wall.is_some() {
                ray.draw(draw, closest_wall.unwrap());
            }
        }
    }
}
