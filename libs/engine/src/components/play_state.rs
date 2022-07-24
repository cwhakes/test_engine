use crate::input;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum PlayState {
    Playing,
    #[default]
    NotPlaying,
}

impl PlayState {
    pub fn is_playing(&self) -> bool {
        *self == Self::Playing
    }

    pub fn is_not_playing(&self) -> bool {
        *self == Self::NotPlaying
    }

    pub fn set_playing(&mut self) {
        input::show_cursor(false);
        *self = Self::Playing;
    }

    pub fn set_not_playing(&mut self) {
        input::show_cursor(true);
        *self = Self::NotPlaying;
    }

    pub fn toggle(&mut self) {
        match self {
            Self::Playing => self.set_not_playing(),
            Self::NotPlaying => self.set_playing(),
        }
    }
}
