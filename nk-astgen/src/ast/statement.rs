use std::fmt;

use crate::ast::*;
use lexer::tokens_new::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum ASTmemoryspace {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum ASTstatement {
    Import {
        //path: String,
        name: String,
    },
    // A node representing a statement
    // A node representing a function definition
    Function {
        public: bool,
        name: String,
        args: Vec<ASTtypecomp>,
        statements: Vec<AST>,
        return_type: ASTtypename,
    },

    Let {
        name: String,
        type_name: Option<String>,
        value: Token,
    },
    Assign {
        l_var: Token,
        r_var: Token,
    },
    AddAssign {
        l_var: Token,
        r_var: Token,
    },
    SubAssign {
        l_var: Token,
        r_var: Token,
    },
    MulAssign {
        l_var: Token,
        r_var: Token,
    },
    DivAssign {
        l_var: Token,
        r_var: Token,
    },
    RemAssign {
        l_var: Token,
        r_var: Token,
    },
    If {
        condition: Vec<AST>,
        statements: Vec<AST>,
    },
    ElseIf {
        condition: Vec<AST>,
        statements: Vec<AST>,
    },
    Else {
        statements: Vec<AST>,
    },

    For {
        start: ASTtypevalue,
        end: ASTtypevalue,
        value: ASTtypevalue,
        statements: Vec<AST>,
    },
    Print {
        value: Box<AST>,
    },
    Println {
        value: Box<AST>,
    },
    Return {
        value: Box<AST>,
    },
}
impl fmt::Display for ASTstatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTstatement::Import { name } => write!(f, "import {}", name),
            ASTstatement::Function {
                public,
                name,
                args,
                statements,
                return_type,
            } => {
                let args_string = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let statements_string = statements
                    .iter()
                    .map(|statement| statement.to_string())
                    .collect::<Vec<String>>()
                    .join("\n");
                let pub_eval = if *public { "public" } else { "" };
                write!(
                    f,
                    "{} function {}({})  {}\n{{\n{}\n}}",
                    pub_eval, name, args_string, return_type, statements_string
                )
            }
            ASTstatement::Let {
                name,
                type_name,
                value,
            } => {
                write!(
                    f,
                    "let {} : {} = {}",
                    name,
                    type_name.clone().unwrap().to_string(),
                    value
                )
            }
            ASTstatement::Assign { l_var, r_var } => {
                write!(f, "{} = {}", l_var, r_var)
            }
            ASTstatement::AddAssign { l_var, r_var } => {
                write!(f, "{} += {}", l_var, r_var)
            }
            ASTstatement::SubAssign { l_var, r_var } => {
                write!(f, "{} -= {}", l_var, r_var)
            }
            ASTstatement::MulAssign { l_var, r_var } => {
                write!(f, "{} *= {}", l_var, r_var)
            }
            ASTstatement::DivAssign { l_var, r_var } => {
                write!(f, "{} /= {}", l_var, r_var)
            }
            ASTstatement::RemAssign { l_var, r_var } => {
                write!(f, "{} %= {}", l_var, r_var)
            }
            ASTstatement::If {
                condition,
                statements,
            } => {
                write!(
                    f,
                    "if {} {{\n{}\n}}",
                    condition
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n"),
                    statements
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            ASTstatement::ElseIf {
                condition,
                statements,
            } => {
                write!(
                    f,
                    "else if {} {{\n{}\n}}",
                    condition
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n"),
                    statements
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            ASTstatement::Else { statements } => {
                write!(
                    f,
                    "else {{\n{}\n}}",
                    statements
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            ASTstatement::For {
                start,
                end,
                value,
                statements,
            } => {
                write!(
                    f,
                    "for {} {} {} {{\n{}\n}}",
                    start.to_string(),
                    end.to_string(),
                    value.to_string(),
                    statements
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            ASTstatement::Print { value } => write!(f, "print {}", value),
            ASTstatement::Println { value } => write!(f, "print {} \\n", value),
            ASTstatement::Return { value } => write!(f, "return {}", value),
        }
    }
}
