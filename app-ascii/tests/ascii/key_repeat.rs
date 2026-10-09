use std::time::Duration;

use app_ascii::{KeyRepeat, KeyRepeatError};

const INITIAL_DELAY: Duration = Duration::from_millis(300);
const INTERVAL: Duration = Duration::from_millis(50);

#[test]
fn should_step_once_as_soon_as_the_key_is_pressed() {
    let mut repeat = KeyRepeat::new(INITIAL_DELAY, INTERVAL).unwrap();

    repeat.tick(true, millis(16));

    assert_eq!(1, repeat.steps_due());
}

#[test]
fn should_not_step_while_the_key_is_up() {
    let mut repeat = KeyRepeat::new(INITIAL_DELAY, INTERVAL).unwrap();

    repeat.tick(false, millis(16));

    assert_eq!(0, repeat.steps_due());
}

#[test]
fn should_wait_for_the_initial_delay_while_the_key_is_held() {
    let mut repeat = KeyRepeat::new(INITIAL_DELAY, INTERVAL).unwrap();
    repeat.tick(true, millis(16));

    repeat.tick(true, millis(299));

    assert_eq!(0, repeat.steps_due());
}

#[test]
fn should_repeat_once_per_interval_after_the_initial_delay() {
    let mut repeat = KeyRepeat::new(INITIAL_DELAY, INTERVAL).unwrap();
    repeat.tick(true, millis(16));

    let steps: Vec<u32> = [300, 49, 1, 100, 10]
        .into_iter()
        .map(|elapsed| {
            repeat.tick(true, millis(elapsed));
            repeat.steps_due()
        })
        .collect();

    assert_eq!(vec![1, 0, 1, 2, 0], steps);
}

#[test]
fn should_start_over_once_the_key_is_released() {
    let mut repeat = KeyRepeat::new(INITIAL_DELAY, INTERVAL).unwrap();
    repeat.tick(true, millis(16));
    repeat.tick(true, millis(280));
    repeat.tick(false, millis(16));

    repeat.tick(true, millis(16));
    let step_on_press = repeat.steps_due();
    repeat.tick(true, millis(280));
    let steps_before_delay = repeat.steps_due();

    assert_eq!((1, 0), (step_on_press, steps_before_delay));
}

#[test]
fn should_reject_a_zero_interval() {
    assert_eq!(
        Err(KeyRepeatError::ZeroInterval),
        KeyRepeat::new(INITIAL_DELAY, Duration::ZERO)
    );
}

fn millis(milliseconds: u64) -> Duration {
    Duration::from_millis(milliseconds)
}
