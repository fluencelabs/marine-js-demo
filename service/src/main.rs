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

pub fn subtract(n: f32) {
    unsafe {
        STATE -= n;
    }
}

pub fn multiply(n: f32) {
    unsafe {
        STATE *= n;
    }
}

pub fn divide(n: f32) {
    unsafe {
        STATE /= n;
    }
}

pub fn reset() {
    unsafe {
        STATE = 0.
    }
}

pub fn get_result() -> f32 {
    unsafe {
        STATE
    }
}

fn main() {}
