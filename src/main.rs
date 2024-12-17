use nannou::prelude::*;
use player::Player;
use wall::Wall;
mod player;
mod ray;
mod wall;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const MIN_RANGE: f32 = -400.0;
const MAX_RANGE: f32 = 400.0;

fn main() {
    nannou::app(model)
        .size(WIDTH, HEIGHT)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    walls: Vec<Wall>,
    player: Player,
}

fn model(_app: &App) -> Model {
    let mut walls: Vec<Wall> = vec![];
    for _i in 0..10 {
        let wall = Wall::new(
            vec2(
                random_range(MIN_RANGE, MAX_RANGE),
                random_range(MIN_RANGE, MAX_RANGE),
            ),
            vec2(
                random_range(MIN_RANGE, MAX_RANGE),
                random_range(MIN_RANGE, MAX_RANGE),
            ),
        );
        walls.push(wall);
    }
    let player = Player::new(vec2(0.0, 0.0));
    Model { walls, player }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse_pos = app.mouse.position();
    model.player.update(mouse_pos);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for wall in &model.walls {
        wall.draw(&draw);
    }

    model.player.draw(&draw, &model.walls);

    draw.to_frame(app, &frame).unwrap();
}
