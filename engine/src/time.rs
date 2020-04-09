use std::time::Instant;

pub struct DeltaT {
    prev_time: Instant,
    curr_time: Instant,
}

impl DeltaT {
    pub fn get(&self) -> f32 {
        self.curr_time.duration_since(self.prev_time).as_secs_f32()
    }

    pub fn update(&mut self) -> &mut Self {
        self.prev_time = self.curr_time;
        self.curr_time = Instant::now();
        self
    }
}

impl Default for DeltaT {
    fn default() -> Self {
        DeltaT {
            prev_time: Instant::now(),
            curr_time: Instant::now(),
        }
    }
}
