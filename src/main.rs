mod game;
mod card;
mod player;
use std::net::*;

fn main() {  

/*    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
  }
  */
    println!("ok");
    let mut game1 = game::Game::new();
    game1.game_loop();
} 
