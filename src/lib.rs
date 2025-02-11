use wasm_bindgen::prelude::*;

// Imports JS fns to Rust
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn msg() -> String {
    alert("Hello from Rust");
    "Hello".to_owned()
}
