#[macro_use]
extern crate lazy_static;

mod interpreter;
mod parser;
mod utils;

use std::sync::Mutex;
use wasm_bindgen::prelude::*;

use crate::interpreter::{Constructor, Interpreter, Type};

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
    let env = &INTERPRETER.lock().unwrap().env;
    let mut json_string = String::new();
    json_string.push_str("[");

    for (i, (s, c)) in env.iter().enumerate() {
        if i != 0 {
            json_string.push_str(",");
        }

        json_string.push_str(&format!("[\"{}\"", s));
        json_string.push_str(", ");
        match c {
            Constructor::UnitLike(name) => {
                json_string.push_str(&format!("\"{}\"]", name));
            }
            Constructor::FunctionLike(_) => todo!(),
        }
    }

    json_string.push_str("]");
    json_string
}

#[wasm_bindgen]
pub fn get_types() -> String {
    let types = &INTERPRETER.lock().unwrap().types;
    let mut json_string = String::new();
    json_string.push_str("[");

    for (i, (s, t)) in types.iter().enumerate() {
        if i != 0 {
            json_string.push_str(", ");
        }

        json_string.push_str(&format!("[\"{}\"", s));
        json_string.push_str(", ");
        match t {
            Type::ScalarType(_, constructors) => {
                json_string.push_str("[");
                for (j, (name, _)) in constructors.iter().enumerate() {
                    if j != 0 {
                        json_string.push_str(", ");
                    }

                    json_string.push_str(&format!("\"{}\"", name.0));
                }
                json_string.push_str("]");
            }
            Type::FunctionType(_, _) => todo!(),
        }
        json_string.push_str("]");
    }

    json_string.push_str("]");
    json_string
}
