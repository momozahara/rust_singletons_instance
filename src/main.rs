mod game;
mod test;

fn main() {

    let game = game::new("Game".to_string());

    println!("{}", game.get_window_name());

    game.set_window_name("Game 2".to_string());

    println!("{}", game.get_window_name());

    let game_instance = game::get_instance();

    println!("{}", game_instance.get_window_name());

    game_instance.set_window_name("Game 3".to_string());

    println!("{}", game_instance.get_window_name());

    test::test();

    test::test_thread();

}