// singeletons
static mut GAME_INSTANCE: Option<Game> = None;

pub struct Game {
    window_name: &'static str,
}

impl Game {
    pub fn get_window_name(&self) -> &'static str {
        self.window_name
    }

    pub fn set_window_name(&mut self, window_name: &'static str) {
        self.window_name = window_name;
    }
}

pub fn new(window_name: &'static str) -> &'static mut Game {
    unsafe {
        if GAME_INSTANCE.is_none() {
            GAME_INSTANCE = Some(Game { window_name });
        }
        GAME_INSTANCE.as_mut().unwrap()
    }
}

pub fn get_instance() -> &'static mut Game {
    unsafe {
        if GAME_INSTANCE.is_none() {
            panic!("Game instance is not initialized");
        }
        GAME_INSTANCE.as_mut().unwrap()
    }
}