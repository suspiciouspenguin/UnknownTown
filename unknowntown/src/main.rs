use macroquad::prelude::*;
mod player;

#[macroquad::main("GAME")]

async fn main (){
    
    pub let mut x: f32 = 10.0;
    pub let mut y: f32 = 10.0;
    pub let speed: f32 = 400.0;

    loop{
        clear_background(WHITE);
        //starting position
        pub let dt = get_frame_time();
        //updated position
        player::player_controls(); 
        let screen_width = screen_width();
        let screen_height = screen_height();
        draw_rectangle(x,y,60.0,60.0,RED);
        next_frame().await;
    }
}
