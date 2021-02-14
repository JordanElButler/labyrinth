use crate::resources::Resources;
use crate::load_assets::load_assets;
use crate::input::InputState;
use crate::timer::Timer;

/*
A singleton to rule all singletons, inject into every function
*/
pub struct Manager {
    pub res: Resources,
    pub input: InputState,
    pub timer: Timer,
}

impl Manager {
    pub fn new() -> Self {
        let mut res = Resources::new();
        load_assets(&mut res);
        let mut timer = Timer::new();
        let mut input = InputState::new();
        Manager {
            res,
            timer,
            input,
        }
    }
    pub fn get_res(&self) -> &Resources {
        &self.res
    }
    pub fn get_timer(&self) -> &Timer {
        &self.timer
    }
    pub fn get_input(&self) -> &InputState {
        &self.input
    }
}