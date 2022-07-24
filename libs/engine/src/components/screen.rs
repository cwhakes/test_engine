use crate::input;
use crate::math::Rect;

#[derive(Default)]
pub struct Screen {
    pub rect: Rect<i32>,
}

impl Screen {
    pub fn aspect_ratio(&self) -> f32 {
        self.rect.width() as f32 / self.rect.height() as f32
    }

    pub fn set_size(&mut self, rect: Rect<i32>) {
        self.rect = rect;
    }

    pub fn center_cursor(&self) {
        input::set_cursor_position((self.rect.center_x(), self.rect.center_y()));
    }
}
