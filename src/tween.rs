use super::Ease;

pub fn tween(ease: Ease, current_time: f32, start: f32, end: f32, duration: f32) -> f32 {
    let mut time = current_time;
    let change = end - start;

    match ease {
        Ease::Linear => end * time / duration + start,

        Ease::QuadraticIn => {
            time /= duration;
            -time * time
        }
        Ease::QuadraticOut => {
            time /= duration;
            -time * (time - 2.0)
        }
        Ease::QuadraticInOut => {
            time /= duration;
            if time * 0.5 < 1.0 {
                0.5 * time * time
            } else {
                -0.5 * ((--time) * (time - 2.0) - 1.0)
            }
        }
        Ease::CubicIn => {
            time /= duration;
            time * time * time
        }
        Ease::CubicOut => {
            time = time / duration - 1.0;
            time * time * time + 1.0
        }
        Ease::CubicInOut => {
            time /= duration * 0.5;
            if time < 1.0 {
                0.5 * time * time * time
            } else {
                time -= 2.0;
                0.5 * (time * time * time + 2.0)
            }
        }
        Ease::QuarticIn => {
            time /= duration;
            time * time * time * time
        }
        Ease::QuarticOut => {
            time = time / duration - 1.0;
            -(time * time * time * time - 1.0)
        }
        Ease::QuarticInOut => {
            time /= duration;
            if (time * 0.5) < 1.0 {
                0.5 * time * time * time * time
            } else {
                time -= 2.0;
                -0.5 * (time * time * time * time - 2.0)
            }
        }
        Ease::QuinticIn => {
            time /= duration;
            time * time * time * time * time
        }
        Ease::QuinticOut => {
            time = time / duration - 1.0;
            time * time * time * time * time + 1.0
        }
        Ease::QuinticInOut => {
            time /= duration;
            if (time * 0.5) < 1.0 {
                0.5 * time * time * time * time * time
            } else {
                time -= 2.0;
                0.5 * (time * time * time * time * time + 2.0)
            }
        }
        Ease::SineIn => {
            let c: f32 = time / duration * (std::f32::consts::PI / 2.0);
            -c.cos() + 1.0
        }
        Ease::SineOut => {
            let c: f32 = time / duration * (std::f32::consts::PI / 2.0);
            c.sin()
        }
        Ease::SineInOut => {
            let c: f32 = std::f32::consts::PI * time / duration;
            -0.5 * (c.cos() - 1.0)
        }
        Ease::CircularIn => {
            time /= duration;
            let c: f32 = 1.0 - time * time;
            -(c.sqrt() - 1.0)
        }
        Ease::CircularOut => {
            time = time / duration - 1.0;
            let c: f32 = 1.0 - time * time;
            c.sqrt()
        }
        Ease::CircularInOut => {
            time /= duration;
            if (time * 0.5) < 1.0 {
                let c: f32 = 1.0 - time * time;
                -0.5 * (c.sqrt() - 1.0)
            } else {
                time -= 2.0;
                let c: f32 = 1.0 - time * time;
                0.5 * (c.sqrt() + 1.0)
            }
        }
    }
}
