use super::vector::Vector;

pub struct Force<T> {
    pub force: Vector<T>,
    // Time in ticks.
    pub time: u64,
    // Enabled, used for forces that should act every tick until disabled.
    pub enabled: bool,
}

impl<T> Force<T> {
    pub fn impulse(force: Vector<T>) -> Force<T> {
        Force {
            force,
            time: 1,
            enabled: false,
        }
    }

    pub fn uniform(force: Vector<T>, time: u64) -> Force<T> {
        Force {
            force,
            time,
            enabled: false,
        }
    }
}
