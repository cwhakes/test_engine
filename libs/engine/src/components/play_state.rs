use crate::input;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum PlayState {
    Playing,
    #[default]
    NotPlaying,
}

impl PlayState {
    pub fn toggle(&mut self) {
        match self {
            Self::Playing => {
                input::show_cursor(true);
                *self = Self::NotPlaying;
            }
            Self::NotPlaying => {
                input::show_cursor(false);
                *self = Self::Playing;
            }
        }
    }
}
