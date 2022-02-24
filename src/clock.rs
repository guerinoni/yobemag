pub struct Clock {
    period: u32,
    counter: u32,
}

impl Clock {
    pub fn new(period: u32) -> Self {
        Self { period, counter: 0 }
    }

    pub fn step(&mut self, cycles: u32) -> u8 {
        self.counter += cycles;
        let rs = self.counter / self.period;
        self.counter = self.counter % self.period;
        rs as u8
    }

    pub(crate) fn reset_counter(&mut self) {
        self.counter = 0;
    }

    pub(crate) fn set_period(&mut self, new_period: u32) {
        self.period = new_period;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let mut clock = Clock::new(64);
        assert_eq!(clock.step(1), 0);
        assert_eq!(clock.counter, 1);

        assert_eq!(clock.step(2), 0);
        assert_eq!(clock.counter, 3);

        assert_eq!(clock.step(4), 0);
        assert_eq!(clock.counter, 7);

        assert_eq!(clock.step(8), 0);
        assert_eq!(clock.counter, 15);

        assert_eq!(clock.step(12), 0);
        assert_eq!(clock.counter, 27);

        assert_eq!(clock.step(16), 0);
        assert_eq!(clock.counter, 43);
    }
}
