/// This file encapsulates work with State just to hide unsafes to provide more "presentable"
/// interface for demo.

static mut STATE: f64 = 0.;

pub fn get_state() -> f64 {
    unsafe { STATE }
}

pub fn set_state(new_state: f64) {
    unsafe { STATE = new_state }
}
