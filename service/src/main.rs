mod calc;
mod state;

use marine_rs_sdk::WasmLoggerBuilder;

fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}
