pub struct Timer {
    duration: i32,
}

impl Timer {
    pub fn new(duration: i32) -> Self {
        Self { duration }
    }

    pub fn tick(&mut self) -> bool {
        if self.duration > 0 {
            self.duration -= 1;
        }
        self.duration <= 0
    }

    pub fn is_done(&self) -> bool {
        self.duration <= 0
    }
}
