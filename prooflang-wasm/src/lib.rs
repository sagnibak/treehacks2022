#[macro_use]
extern crate lazy_static;

mod interpreter;
mod parser;
mod utils;

use std::sync::Mutex;
use wasm_bindgen::prelude::*;

use crate::interpreter::{Interpreter, Type};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// The interpreter.
lazy_static! {
    static ref INTERPRETER: Mutex<Interpreter> = Mutex::new(Interpreter::new());
}

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
    INTERPRETER.lock().unwrap().interpret(input.to_owned())
}

#[wasm_bindgen]
pub fn get_env() -> String {
    format!("{:?}", &INTERPRETER.lock().unwrap().env)
}

#[wasm_bindgen]
pub fn get_types() -> String {
    format!("{:?}", &INTERPRETER.lock().unwrap().types)
}
