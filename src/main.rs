mod game;
mod card;
mod player;


fn main() {  
    println!("ok");
    let mut game1 = game::Game::new();
    game1.game_loop();
} 
