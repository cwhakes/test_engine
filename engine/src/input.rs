use crate::math::Point;

use std::collections::HashMap;
use std::sync::Mutex;

use winapi::shared::windef;
use winapi::um::winuser;

lazy_static! {
    pub static ref INPUT: Mutex<Input> = Mutex::new(Default::default());
}

pub trait Listener {
    fn name(&self) -> String;
    fn on_key_down(&mut self, _key: usize) {}
    fn on_key_up(&mut self, _key: usize) {}

    fn on_mouse_move(&mut self, _pos: Point) {}
    fn on_left_mouse_down(&mut self) {}
    fn on_right_mouse_down(&mut self) {}
    fn on_left_mouse_up(&mut self) {}
    fn on_right_mouse_up(&mut self) {}
}

impl<T: Listener> Listener for Option<T> {
    fn name(&self) -> String {
        if let Some(lis) = self {
            lis.name()
        } else {
            "".to_string()
        }
    }
    fn on_key_down(&mut self, key: usize) {
        if let Some(lis) = self {
            lis.on_key_down(key)
        }
    }
    fn on_key_up(&mut self, key: usize) {
        if let Some(lis) = self {
            lis.on_key_up(key)
        }
    }
    fn on_mouse_move(&mut self, pos: Point) {
        if let Some(lis) = self {
            lis.on_mouse_move(pos)
        }
    }
    fn on_left_mouse_down(&mut self) {
        if let Some(lis) = self {
            lis.on_left_mouse_down()
        }
    }
    fn on_right_mouse_down(&mut self) {
        if let Some(lis) = self {
            lis.on_right_mouse_down()
        }
    }
    fn on_left_mouse_up(&mut self) {
        if let Some(lis) = self {
            lis.on_left_mouse_up()
        }
    }
    fn on_right_mouse_up(&mut self) {
        if let Some(lis) = self {
            lis.on_right_mouse_up()
        }
    }
}

pub struct Input {
    hashmap: HashMap<String, &'static Mutex<dyn Listener + Send + Sync>>,
    keys_state: [u8; 256],
    old_keys_state: [u8; 256],
    old_mouse_pos: Point,
    pub original_mouse_pos: Option<Point>,
}

impl Input {
    pub fn add_listener(&mut self, listener: &'static Mutex<dyn Listener + Send + Sync>) {
        let name = listener.lock().unwrap().name();
        self.hashmap.insert(name, listener);
    }

    pub fn remove_listener(&mut self, listener: &'static Mutex<dyn Listener + Send + Sync>) {
        self.hashmap.remove(&listener.lock().unwrap().name());
    }

    pub fn update(&mut self) {
        unsafe {
            if 0 < winuser::GetKeyboardState(self.keys_state.as_mut_ptr()) {
                for mut lis in self.hashmap.values().map(|lis| lis.lock().unwrap()) {
                    for (index, (state, old_state)) in self.keys_state.iter().zip(
                        self.old_keys_state.iter()
                    ).enumerate() {
                        //Check first bit
                        if 0 < (state & 0xf0) {
                            if index as i32 == winuser::VK_LBUTTON {
                                if state != old_state {
                                    lis.on_left_mouse_down();
                                }
                            } else if index as i32 == winuser::VK_RBUTTON {
                                if state != old_state {
                                    lis.on_right_mouse_down();
                                }
                            } else {
                                lis.on_key_down(index);
                            }
                        } else if state != old_state {
                            if index as i32 == winuser::VK_LBUTTON {
                                lis.on_left_mouse_up();
                            } else if index as i32 == winuser::VK_RBUTTON {
                                lis.on_right_mouse_up();
                            } else {
                                lis.on_key_up(index);
                            }
                        }
                    }
                }
            }
            self.old_keys_state = self.keys_state;

            let new_mouse_pos = get_cursor_position();
            if new_mouse_pos != self.old_mouse_pos {
                self.hashmap.values().for_each(|lis| {
                    lis.lock().unwrap().on_mouse_move(new_mouse_pos)
                });
            }
            self.old_mouse_pos = new_mouse_pos;
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Input {
            hashmap: Default::default(),
            keys_state: [0; 256],
            old_keys_state: [0; 256],
            old_mouse_pos: get_cursor_position(),
            original_mouse_pos: Default::default(),
        }
    }
}

fn get_cursor_position() -> Point {
    let mut point = windef::POINT::default();
    unsafe { winuser::GetCursorPos(&mut point); }
    point.into()
}

pub fn set_cursor_position(pos: impl Into<Point>) {
    unsafe {
        let pos = pos.into();
        winuser::SetCursorPos(pos.x, pos.y)
    };
}

pub fn show_cursor(show: bool) {
    unsafe {
        let b_show = if show {1} else {0};
        winuser::ShowCursor(b_show);
    }
}
