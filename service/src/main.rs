use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

static mut STATE: f32 = 0.;

#[marine]
pub fn add(n: f32) {
    unsafe {
        STATE += n;
    }
}

#[marine]
pub fn subtract(n: f32) {
    unsafe {
        STATE -= n;
    }
}

#[marine]
pub fn multiply(n: f32) {
    unsafe {
        STATE *= n;
    }
}

#[marine]
pub fn divide(n: f32) {
    unsafe {
        STATE /= n;
    }
}

#[marine]
pub fn reset() {
    unsafe {
        STATE = 0.
    }
}

#[marine]
pub fn get_result() -> f32 {
    unsafe {
        STATE
    }
}

fn main() {}
