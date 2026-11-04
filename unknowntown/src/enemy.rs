use macroquad::prelude::*;

struct Enemy{
    x:f32,
    y:f32,
    width:f32,
    height:f32,
}
 
pub fn Create_enemy(x_cod:f32,y_cod:f32){
    let enemy= Enemy{
                 x:x_cod,
                 y:y_cod,
                 width:60.0,
                 height:60.0,
    };
        draw_rectangle(
            enemy.x,
            enemy.y,
            enemy.width,
            enemy.height,
            RED
            );
    
}
