mod first_look;
use self::first_look::FirstLook;

pub struct State {
    pub first_look: FirstLook,
}

impl State {
    pub fn new() -> Self {
        Self { first_look: FirstLook::default() }
    }
}
