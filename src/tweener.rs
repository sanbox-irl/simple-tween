use super::Ease;

pub struct Tweener {
    pub value: f32,
    pub alive: bool,

    end_value: f32,
    time: f32,
    duration: f32,
    ease: Ease,
}

impl Tweener {
    pub fn new(value: f32, end_value: f32, duration: f32, ease: Ease) -> Tweener {
        Tweener {
            value,
            end_value,
            duration,
            ease,
            alive: true,
            time: 0.0,
        }
    }

    pub fn tick(&mut self, delta_time: f32) {
        if self.time >= 1.0 {
            self.alive = false;
        } else {
            self.time += delta_time;

            let value = self.tock(&self.ease, self.time, self.duration);
            let amount = self.value.lerp(&self.end_value, value);
            self.value = amount;
        }
    }

    pub fn set_ease(&mut self, ease: Ease) {
        self.ease = ease;
    }
}
