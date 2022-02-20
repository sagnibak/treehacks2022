use std::collections::{HashMap, HashSet};

use crate::parser::{parse_set, Expr, SetDefExpr};

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
    ScalarType(Name, HashMap<Name, Constructor>),
    FunctionType(Box<Type>, Box<Type>),
}

impl Type {
    fn instantiate(&self, name: &Name) -> Constructor {
        match self {
            Self::ScalarType(_, constructors) => constructors.get(name).unwrap().clone(),
            Self::FunctionType(arg_type, out_type) => todo!(),
        }
    }
}

pub struct Interpreter {
    pub types: HashMap<String, Type>,
    pub env: HashMap<String, Constructor>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            env: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, code: String) -> String {
        let parse_result = parse_set(&code);
        if let Ok((_, expr)) = parse_result {
            let new_type = match expr {
                Expr::SetDef(SetDefExpr {
                    name,
                    constructor_names,
                }) => Type::ScalarType(
                    Name(name),
                    constructor_names
                        .into_iter()
                        .map(|x| (Name(x.clone()), Constructor::UnitLike(x)))
                        .collect(),
                ),
                Expr::FuncDef(_) => todo!(),
            };
            match &new_type {
                Type::ScalarType(name, _) => {
                    format!("Defined new type: {}", name.0)
                },
                Type::FunctionType(..) => todo!(),
            }
        } else {
            format!("Received error: {}", parse_result.unwrap_err())
        }
    }
}
