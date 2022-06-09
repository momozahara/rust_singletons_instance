use crate::game;

pub fn test() {

    let game = game::get_instance();

    game.set_window_name("Game 4");

    println!("{}", game.get_window_name());

}