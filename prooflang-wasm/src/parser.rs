#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetDefExpr {
    pub name: String,
    pub constructor_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarDefExpr {
    pub name: String,
    pub type_name: String,
    pub constructor: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDefExpr {
    cases: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    SetDef(SetDefExpr),
    VarDef(VarDefExpr),
    FuncDef(FunctionDefExpr),
}

fn parse_tag<'s>(tag: &'static str) -> Box<dyn Fn(&'s str) -> Result<(), String>> {
    Box::new(move |s: &'s str| {
        if s == tag {
            Ok(())
        } else {
            Err(format!("Tried to parse tag {}, got {}", tag, s))
        }
    })
}

pub fn parse_set(s: &str) -> Result<(String, Expr), String> {
    let mut splitted = s.split_whitespace();
    parse_tag("set")(splitted.next().unwrap_or("Empty"))?;
    let name = if let Some(_name) = splitted.next() {
        _name
    } else {
        return Err("Expected identifier (type name)".to_owned());
    };
    parse_tag("=")(splitted.next().unwrap_or("Missing `=`"))?;
    parse_tag("{")(splitted.next().unwrap_or("Missing `{`"))?;

    let mut constructor_names = Vec::new();
    let mut next_token = if let Some(token) = splitted.next() {
        token
    } else {
        return Err("Unexpected end of set definition expression".to_owned());
    };
    while next_token != "}" {
        constructor_names.push(next_token.to_owned());
        next_token = if let Some(token) = splitted.next() {
            token
        } else {
            return Err("Unexpected end of set definition expression".to_owned());
        };
    }

    Ok((
        splitted.map(|x| String::from(" ") + x).collect(),
        Expr::SetDef(SetDefExpr {
            name: name.to_owned(),
            constructor_names,
        }),
    ))
}

pub fn parse_var(s: &str) -> Result<(String, Expr), String> {
    let mut splitted = s.split_whitespace();

    parse_tag("let")(splitted.next().unwrap_or("Empty"))?;

    let name = if let Some(_name) = splitted.next() {
        _name.to_owned()
    } else {
        return Err("Expected identifier (variable name)".to_owned());
    };

    parse_tag("=")(splitted.next().unwrap_or("Missing `=`"))?;

    let type_name = if let Some(_name) = splitted.next() {
        _name.to_owned()
    } else {
        return Err("Expected identifier (type name)".to_owned());
    };

    parse_tag("->")(splitted.next().unwrap_or("Missing `->`"))?;

    let constructor = if let Some(_name) = splitted.next() {
        _name.to_owned()
    } else {
        return Err("Expected identifier (constructor name)".to_owned());
    };

    Ok((
        splitted.map(|x| String::from(" ") + x).collect(),
        Expr::VarDef(VarDefExpr {
            name: name,
            type_name,
            constructor,
        }),
    ))
}

pub fn parse(s: &str) -> Result<(String, Expr), String> {
    let parse_result = parse_set(s);
    if parse_result.is_err() {
        parse_var(s)
    } else {
        parse_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        assert_eq!(
            parse_set("set Bottom = { }"),
            Ok((
                String::from(""),
                Expr::SetDef(SetDefExpr {
                    name: String::from("Bottom"),
                    constructor_names: Vec::new(),
                })
            ))
        );

        assert_eq!(
            parse_set("set Unit = { Uno }"),
            Ok((
                String::from(""),
                Expr::SetDef(SetDefExpr {
                    name: String::from("Unit"),
                    constructor_names: vec![String::from("Uno")],
                })
            ))
        );

        assert_eq!(
            parse_set("set Bool = { False True }"),
            Ok((
                String::from(""),
                Expr::SetDef(SetDefExpr {
                    name: String::from("Bool"),
                    constructor_names: vec![String::from("False"), String::from("True")],
                })
            ))
        );
    }

    #[test]
    fn test_parse_set_with_trailing() {
        assert_eq!(
            parse_set("set Bottom = { } this is useless stuff that won't get parsed"),
            Ok((
                String::from(" this is useless stuff that won't get parsed"),
                Expr::SetDef(SetDefExpr {
                    name: String::from("Bottom"),
                    constructor_names: Vec::new(),
                })
            ))
        );

        assert_eq!(
            parse_set("set Unit = { Uno } this is useless stuff that won't get parsed"),
            Ok((
                String::from(" this is useless stuff that won't get parsed"),
                Expr::SetDef(SetDefExpr {
                    name: String::from("Unit"),
                    constructor_names: vec![String::from("Uno")],
                })
            ))
        );

        assert_eq!(
            parse_set("set Bool = { False True } this is useless stuff that won't get parsed"),
            Ok((
                String::from(" this is useless stuff that won't get parsed"),
                Expr::SetDef(SetDefExpr {
                    name: String::from("Bool"),
                    constructor_names: vec![String::from("False"), String::from("True")],
                })
            ))
        );
    }
}
