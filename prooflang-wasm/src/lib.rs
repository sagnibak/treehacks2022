mod interpreter;
mod utils;

use wasm_bindgen::prelude::*;

use crate::interpreter::{Interpreter, Type};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// The interpreter.
static mut INTERPRETER: Interpreter = Interpreter::new();

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, prooflang-wasm!");
}

#[wasm_bindgen]
pub fn interpret(input: &str) -> String {
    unsafe { INTERPRETER.interpret(input.to_owned()) }
}

#[wasm_bindgen]
pub fn get_env() -> String {
    format!("{:?}", unsafe { &INTERPRETER.env })
}
