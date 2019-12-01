use super::Ease;
use std::f32::consts::PI;

pub fn tween(ease: Ease, current_time: f32, start: f32, end: f32, duration: f32) -> f32 {
    let mut time = current_time;
    let change = end - start;

    let tweened_amount = match ease {
        Ease::Linear => end * time / duration,

        // Quadratic
        Ease::QuadraticIn => {
            time /= duration;
            change * time * time
        }
        Ease::QuadraticOut => {
            time /= duration;
            -change * (time - 2.0)
        }
        Ease::QuadraticInOut => {
            time /= duration / 2.0;
            if time < 1.0 {
                // Quadratic In
                time * time * change / 2.0
            } else {
                // Quadratic Out
                -change / 2.0 * (time * (time - 2.0) - 1.0)
            }
        }

        // Cubic
        Ease::CubicIn => {
            time /= duration;
            change * time * time * time
        }
        Ease::CubicOut => {
            time = time / duration - 1.0;
            change * time * time * time + 1.0
        }
        Ease::CubicInOut => {
            time /= duration / 2.0;

            if time < 1.0 {
                change / 2.0 * time * time * time
            } else {
                time -= 2.0;
                change / 2.0 * (time * time * time + 2.0)
            }
        }

        // Quartic
        Ease::QuarticIn => {
            time /= duration;
            change * time * time * time * time
        }
        Ease::QuarticOut => {
            time = time / duration - 1.0;
            -change * (time * time * time * time - 1.0)
        }
        Ease::QuarticInOut => {
            time /= duration / 2.0;

            if time < 1.0 {
                change / 2.0 * time * time * time * time
            } else {
                time -= 2.0;
                -change / 2.0 * (time * time * time * time - 2.0)
            }
        }

        // Quintic
        Ease::QuinticIn => {
            time /= duration;
            change * time * time * time * time * time
        }
        Ease::QuinticOut => {
            time = time / duration - 1.0;
            change * time * time * time * time * time + 1.0
        }
        Ease::QuinticInOut => {
            time /= duration / 2.0;
            if time < 1.0 {
                change / 2.0 * time * time * time * time * time
            } else {
                time -= 2.0;
                change / 2.0 * (time * time * time * time * time + 2.0)
            }
        }

        // Sinusidal
        Ease::SineIn => change * (1.0 - f32::cos(time / duration * PI / 2.0)),
        Ease::SineOut => change * f32::sin(time / duration * PI / 2.0),
        Ease::SineInOut => change / 2.0 * (1.0 - f32::cos(PI * time / duration)),

        // Circular
        Ease::CircularIn => {
            time /= duration;
            change * (1.0 - f32::sqrt(1.0 - time * time))
        }
        Ease::CircularOut => {
            time = time / duration - 1.0;
            change * f32::sqrt(1.0 - time * time)
        }
        Ease::CircularInOut => {
            time /= duration / 2.0;
            if time < 1.0 {
                change / 2.0 * (1.0 - f32::sqrt(1.0 - time * time))
            } else {
                time -= 2.0;
                change / 2.0 * f32::sqrt(1.0 - time * time + 1.0)
            }
        }
    };

    tweened_amount + start
}
