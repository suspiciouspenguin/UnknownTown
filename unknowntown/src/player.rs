use macroquad::prelude::*;

pub struct Player{
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    speed: f32,
    color: Color,
}

impl Player{
    pub fn new() -> Player{
        Player{
            width: 80.0,
            height: 80.0,
            x: 100.0,
            y: 100.0,
            speed: 120.0,
            color: RED
        }
    }

    pub fn update(&mut self) {

        let dt = get_frame_time();

        if is_key_down(KeyCode::D) {
            self.x += self.speed * dt;
        }

        if is_key_down(KeyCode::A) {
            self.x -= self.speed * dt;
        }

        if is_key_down(KeyCode::W) {
            self.y -= self.speed * dt;
        }

        if is_key_down(KeyCode::S) {
            self.y += self.speed * dt;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.x,
            self.y,
            self.width,
            self.height,
            self.color,
        );
    }
    
}
