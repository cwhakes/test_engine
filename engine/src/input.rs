use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use winapi::um::winuser;

lazy_static! {
    pub static ref INPUT: Mutex<Input> = Mutex::new(Default::default());
}

pub trait Listener {
    fn name(&self) -> &'static str;
    fn on_key_down(&mut self, key: usize);
    fn on_key_up(&mut self, key: usize);
}

pub struct Input {
    hashmap: HashMap<String, Arc<Mutex<dyn Listener + Send + Sync>>>,
    keys_state: [u8; 256],
    old_keys_state: [u8; 256],
}

impl Input {
    pub fn add_listener(&mut self, listener: Arc<Mutex<dyn Listener + Send + Sync>>) {
        let name = listener.lock().unwrap().name().to_string();
        self.hashmap.insert(name, listener);
    }

    pub fn remove_listener(&mut self, listener: Arc<Mutex<dyn Listener + Send + Sync>>) {
        self.hashmap.remove(listener.lock().unwrap().name());
    }

    pub fn update(&mut self) {
        unsafe {
            if 0 < winuser::GetKeyboardState(self.keys_state.as_mut_ptr()) {
                for (index, (state, old_state)) in self.keys_state.iter().zip(
                    self.old_keys_state.iter()
                ).enumerate() {
                    //Check first bit
                    if 0 < (state & 0xf0) {
                        self.hashmap.values().for_each(|lis| lis.lock().unwrap().on_key_down(index));
                    } else if state != old_state {
                        self.hashmap.values().for_each(|lis| lis.lock().unwrap().on_key_up(index));
                    }
                }
            }
            self.old_keys_state = self.keys_state;
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Input {
            hashmap: Default::default(),
            keys_state: [0; 256],
            old_keys_state: [0; 256],
        }
    }
}
