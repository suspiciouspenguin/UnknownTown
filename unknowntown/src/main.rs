use macroquad::prelude::*;

#[macroquad::main("GAME")]

async fn main (){
    
    let mut x: f32 = 10.0;
    let mut y: f32 = 10.0;
    let speed: f32 = 400.0;

    loop{
        clear_background(WHITE);
        //starting position
        let dt = get_frame_time();
        //updated position
        if is_key_down(KeyCode::W) {y -= speed*dt};        
        if is_key_down(KeyCode::S) {y += speed*dt};        
        if is_key_down(KeyCode::A) {x -= speed*dt};        
        if is_key_down(KeyCode::D) {x += speed*dt};        
        let screen_width = screen_width();
        let screen_height = screen_height();
        draw_rectangle(x,y,60.0,60.0,RED);
        next_frame().await;
    }
}
