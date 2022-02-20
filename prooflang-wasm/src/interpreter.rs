use std::collections::{HashMap, HashSet};

use crate::parser::{parse, Expr, SetDefExpr, VarDefExpr};

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
        let parse_result = parse(&code);
        if let Ok((_, expr)) = parse_result {
            match expr {
                Expr::SetDef(SetDefExpr {
                    name,
                    constructor_names,
                }) => {
                    let new_type = Type::ScalarType(
                        Name(name.clone()),
                        constructor_names
                            .into_iter()
                            .map(|x| (Name(x.clone()), Constructor::UnitLike(x)))
                            .collect(),
                    );
                    self.types.insert(name.clone(), new_type.clone());
                    format!("Defined new type: {}", name)
                }

                Expr::VarDef(VarDefExpr {
                    name,
                    type_name,
                    constructor,
                }) => {
                    let type_ = if let Some(type_) = self.types.get(&type_name) {
                        type_
                    } else {
                        return format!("Undefined type: {}", type_name);
                    };
                    let val = type_.instantiate(&Name(constructor));
                    self.env.insert(name.clone(), val);
                    format!("Defined new type: {}", name)
                }

                Expr::FuncDef(_) => String::from("not implemented"),
            }
        } else {
            format!("Received error: {}", parse_result.unwrap_err())
        }
    }
}
