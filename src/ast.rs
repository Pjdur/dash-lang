use std::collections::HashMap;

/// Stores the runtime context for the interpreter, including variables and user-defined functions.
#[derive(Default)]
pub struct Context {
    /// A map of variable names to their string values.
    pub variables: HashMap<String, String>,
    /// A map of function names to their parameter list and body.
    pub functions: HashMap<String, (Vec<String>, Vec<Stmt>)>,
}

/// Represents an expression in the language.
#[derive(Debug, Clone)]
pub enum Expr {
    /// An integer literal.
    Int(i64),
    /// A string literal.
    Str(String),
    /// A variable reference.
    Var(String),
    /// A function call with arguments.
    Call(String, Vec<Expr>),
    /// A binary operation (e.g., addition, comparison).
    Binary(Box<Expr>, Op, Box<Expr>),
}

/// Represents a statement in the language.
#[derive(Debug, Clone)]
pub enum Stmt {
    /// Prints the result of an expression.
    Print(Expr),
    /// Declares or updates a variable.
    Let(String, Expr),
    /// Conditional execution.
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    /// Looping construct.
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    /// Exits a loop early.
    Break,
    /// Skips to the next loop iteration.
    Continue,
    /// Defines a function.
    Fn {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    /// Calls a function as a statement.
    Call(String, Vec<Expr>),
    /// Returns a value from a function.
    Return(Expr),
}

/// Supported binary operators.
#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    Equal,
    NotEqual,
}

/// Internal control flow used during execution.
pub enum LoopControl {
    None,
    Break,
    Continue,
    Return(String),
}