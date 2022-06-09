use crate::game;

use std::thread;

pub fn test() {

    let game = game::get_instance();

    game.set_window_name("Game 4".to_string());

    // print Game 4 in thread test

}

fn work_thread() {

    let game = game::get_instance();

    println!("{}", game.get_window_name());

    game.set_window_name("I AM WORK THREAD TAKING OVER THE GAME".to_string());

    println!("{}", game.get_window_name());

}

pub fn test_thread() {

    let handle = thread::spawn(|| {
        work_thread();
    });

    handle.join().unwrap();

}