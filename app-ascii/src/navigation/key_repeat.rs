use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum KeyRepeatError {
    ZeroInterval,
}

/// Turns a held key into steps: one on the press, then, after an initial delay,
/// one per interval, like a keyboard's auto-repeat.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyRepeat {
    initial_delay: Duration,
    interval: Duration,
    held_for: Option<Duration>,
    steps_due: u32,
}

impl KeyRepeat {
    /// # Errors
    ///
    /// Fails when `interval` is zero.
    pub fn new(initial_delay: Duration, interval: Duration) -> Result<Self, KeyRepeatError> {
        if interval.is_zero() {
            return Err(KeyRepeatError::ZeroInterval);
        }

        Ok(Self {
            initial_delay,
            interval,
            held_for: None,
            steps_due: 0,
        })
    }

    pub fn tick(&mut self, held: bool, elapsed: Duration) {
        (self.held_for, self.steps_due) = match (held, self.held_for) {
            (false, _) => (None, 0),
            (true, None) => (Some(Duration::ZERO), 1),
            (true, Some(held_for)) => {
                let now_held_for = held_for + elapsed;
                let repeats = self.repeats_after(now_held_for) - self.repeats_after(held_for);
                (Some(now_held_for), repeats)
            }
        };
    }

    #[must_use]
    pub fn steps_due(&self) -> u32 {
        self.steps_due
    }

    fn repeats_after(&self, held_for: Duration) -> u32 {
        match held_for.checked_sub(self.initial_delay) {
            None => 0,
            Some(past_delay) => (past_delay.as_nanos() / self.interval.as_nanos()) as u32 + 1,
        }
    }
}
