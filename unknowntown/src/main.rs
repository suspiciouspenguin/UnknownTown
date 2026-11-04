use macroquad::prelude::*;
mod enemy;

#[macroquad::main("GAME")]

async fn main (){
    
    loop{
        clear_background(WHITE);
        enemy::Create_enemy(30.0,30.0);
        enemy::Create_enemy(70.0,70.0);
        next_frame().await;
    }
}
