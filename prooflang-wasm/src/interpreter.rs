use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Name(String);

#[derive(Clone, Debug)]
pub struct Function {
    args: Vec<Type>,
    body: HashMap<Constructor, Constructor>,
}

#[derive(Clone, Debug)]
pub enum Constructor {
    UnitLike(String),
    FunctionLike(Function),
}

#[derive(Clone, Debug)]
pub enum Type {
    ScalarType(HashMap<Name, Constructor>),
    FunctionType(Box<Type>, Box<Type>),
}

impl Type {
    fn instantiate(&self, name: &Name) -> Constructor {
        match self {
            Self::ScalarType(hmap) => hmap.get(name).unwrap().clone(),
            Self::FunctionType(arg_type, out_type) => todo!(),
        }
    }
}

pub struct Interpreter {
    types: HashMap<String, Type>,
    pub env: HashMap<String, Constructor>,
}

impl Interpreter {
    pub const fn new() -> Self {
        Self {
            types: HashMap::new(),
            env: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, code: String) -> String {
        format!("This is what WASM received: {}", code)
    }
}
