// singeletons
static mut GAME_INSTANCE: Option<Game> = None;

pub struct Game {
    window_name: String,
}

impl Game {
    pub fn get_window_name(&self) -> String {
        self.window_name.clone()
    }

    pub fn set_window_name(&mut self, window_name: String) {
        self.window_name = window_name;
    }
}

pub fn new(window_name: String) -> &'static mut Game {
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