<<<<<<< HEAD
use macroquad::prelude::*;
mod enemy;
=======
mod player;
>>>>>>> c84ca93a1f53ab4cf3653a8ff26463175375391d

use macroquad::prelude::*;
use player::Player;

#[macroquad::main("GAME")]

async fn main (){
<<<<<<< HEAD
    
    loop{
        clear_background(WHITE);
        enemy::Create_enemy(30.0,30.0);
        enemy::Create_enemy(70.0,70.0);
=======

    let mut player = Player::new();
    loop{
        clear_background(WHITE);

        //updated position
        player.update();
        player.draw();
        
>>>>>>> c84ca93a1f53ab4cf3653a8ff26463175375391d
        next_frame().await;
    }
}
