use macroquad::prelude::*;

pub struct player = {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    speed: f32, 
}
let screen_height = screen_height();
let mut Player_control = player{
    width: 60,
    height: 60,
    x: 0,
    y: screen_height - height,
    speed: 120,
}
pub fn player_pos(dt: f32){
    if is_key_down(KeyCode::W) {y -= Player_control.speed * dt};
    if is_key_down(KeyCode::S) {y += Player_control.speed * dt};
    if is_key_down(KeyCode::D) {x += Player_control.speed * dt};
    if is_key_down(KeyCode::A) {x -= Player_control.speed * dt};
}
