use crate::state::get_state;
use crate::state::set_state;

use log::info;
use log::trace;
use marine_rs_sdk::marine;

#[marine]
pub fn add(num: f64) -> f64 {
    let prev_state = get_state();
    let new_state = prev_state + num;
    set_state(new_state);

    info!("{} + {} = {}", prev_state, num, new_state);

    new_state
}

#[marine]
pub fn subtract(num: f64) -> f64 {
    let prev_state = get_state();
    let new_state = prev_state - num;
    set_state(new_state);

    info!("{} - {} = {}", prev_state, num, new_state);

    new_state
}

#[marine]
pub fn multiply(num: f64) -> f64 {
    let prev_state = get_state();
    let new_state = prev_state * num;
    set_state(new_state);

    info!("{} * {} = {}", prev_state, num, new_state);

    new_state
}

#[marine]
pub fn divide(num: f64) -> f64 {
    let prev_state = get_state();
    if num == 0f64 {
        return prev_state;
    }

    let new_state = prev_state / num;
    set_state(new_state);

    info!("{} / {} = {}", prev_state, num, new_state);

    new_state
}

#[marine]
pub fn clear_state() {
    trace!("clear_state() is called");

    set_state(0f64);
}

#[marine]
pub fn state() -> f64 {
    trace!("state() is called");

    get_state()
}
