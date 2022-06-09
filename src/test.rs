use crate::game;

pub fn test() {

    let mut _game = game::get_instance();

    _game.set_window_name("Game 4");

    println!("{}", _game.get_window_name());

}