use macroquad::prelude::*;
mod player;

use macroquad::prelude::*;
use player::Player;

#[macroquad::main("GAME")]

async fn main (){
       let mut player = Player::new(); 
    loop{
        clear_background(WHITE);

        //updated position
        player.draw_cod();
        player.update();
        player.draw();
        
        next_frame().await;
    }
}
