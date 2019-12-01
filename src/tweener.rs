use super::{tween, Ease};

#[derive(Default)]
pub struct Tweener {
    pub current_value: f32,
    pub alive: bool,
    pub ease: Ease,

    start_value: f32,
    end_value: f32,
    time: f32,
    duration: f32,

    at_time: Vec<(f32, Box<dyn FnMut()>)>,
    every_update: Vec<Box<dyn FnMut()>>,
    on_complete: Vec<Box<dyn FnMut()>>,
}

impl Tweener {
    pub fn new(value: f32, end_value: f32, duration: f32, ease: Ease) -> Tweener {
        Tweener {
            current_value: value,
            start_value: value,
            end_value,
            duration,
            ease,
            alive: true,
            time: 0.0,
            at_time: vec![],
            every_update: vec![],
            on_complete: vec![],
        }
    }

    // Builders
    pub fn on_normalized_time(&mut self, time: f32, callback: Box<dyn FnMut()>) -> &mut Self {
        self.at_time.push((time, callback));
        self
    }

    pub fn on_update(&mut self, callback: Box<dyn FnMut()>) -> &mut Self {
        self.every_update.push(callback);
        self
    }

    pub fn on_complete(&mut self, callback: Box<dyn FnMut()>) -> &mut Self {
        self.on_complete.push(callback);
        self
    }

    // Update
    pub fn update(&mut self, delta_time: f32) {
        if self.time >= 1.0 {
            self.alive = false;
        } else {
            self.current_value = tween::tween(
                self.ease,
                self.time,
                self.start_value,
                self.end_value,
                self.duration,
            );
        }

        self.time += delta_time;
    }

    // Getters
    pub fn start_value(&self) -> f32 {
        self.start_value
    }

    pub fn end_value(&self) -> f32 {
        self.end_value
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }
}

use std::fmt;
impl fmt::Debug for Tweener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Tweener {{ current: {}, ease: {}, is_alive: {}, start: {}, end: {}, current_time: {}, duration: {}, at_time_count: {}, every_update_count: {}, on_complete_count: {}}}",
            self.current_value, 
            self.ease, 
            self.alive, 
            self.start_value, 
            self.end_value, 
            self.time, 
            self.duration, 
            self.at_time.len(), 
            self.every_update.len(), 
            self.on_complete.len()
        )
    }
}
