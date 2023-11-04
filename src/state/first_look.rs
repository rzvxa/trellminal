pub struct FirstLook {
    pub selected_button: u8,
}

impl FirstLook {
    pub fn default() -> Self {
        Self { selected_button: 0 }
    }
}
