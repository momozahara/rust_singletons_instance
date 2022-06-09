mod game;
mod test;

fn main() {

    let mut _game = game::new("Game");

    println!("{}", _game.get_window_name());

    _game.set_window_name("Game 2");

    println!("{}", _game.get_window_name());

    let mut _game_instance = game::get_instance();

    println!("{}", _game_instance.get_window_name());

    _game_instance.set_window_name("Game 3");

    println!("{}", _game_instance.get_window_name());

    test::test();

}