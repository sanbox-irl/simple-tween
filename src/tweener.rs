use super::{tween, Ease};

pub struct Tweener {
    pub current_value: f32,
    pub alive: bool,
    pub ease: Ease,

    start_value: f32,
    end_value: f32,
    time: f32,
    duration: f32,
    callbacks: Vec<(f32, Box<dyn FnMut()>)>,
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
            callbacks: vec![],
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;

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
    }
}
