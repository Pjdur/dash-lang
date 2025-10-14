use pest::Parser;
use pest_derive::Parser;
use crate::eval::exec_stmt;
use crate::ast::{Stmt, Expr, Op, Context};

#[derive(Parser)]
#[grammar = "dash.pest"]
pub struct DashParser;

/// Parses and executes a source program written in the custom language.
///
/// This function uses the Pest parser to convert the source string into an AST,
/// then executes each statement in order using a fresh `Context`.
///
/// # Arguments
/// * `source` - A string slice containing the source code to run.
pub(crate) fn run(source: &str) {
    match DashParser::parse(Rule::program, source) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();
            let ast = build_ast(pair.into_inner());
            let mut ctx = Context::default();
            for stmt in ast {
                exec_stmt(&stmt, &mut ctx);
            }
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
}

/// Converts a sequence of Pest pairs into a list of statements (AST).
///
/// Filters out non-statement rules and delegates to `build_stmt` for each.
///
/// # Arguments
/// * `pairs` - Pest pairs representing parsed tokens.
///
/// # Returns
/// A vector of `Stmt` representing the program's abstract syntax tree.
fn build_ast(pairs: pest::iterators::Pairs<Rule>) -> Vec<Stmt> {
    pairs
        .filter_map(|pair| match pair.as_rule() {
            Rule::statement => Some(build_stmt(pair.into_inner())),
            _ => None,
        })
        .collect()
}

/// Builds a single statement from its Pest pair representation.
///
/// Matches the rule type and constructs the corresponding `Stmt` variant.
///
/// # Arguments
/// * `pairs` - Pest pairs representing a statement.
///
/// # Returns
/// A `Stmt` enum variant representing the parsed statement.
fn build_stmt(mut pairs: pest::iterators::Pairs<Rule>) -> Stmt {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::print_stmt => {
            let mut inner = pair.into_inner();
            let expr_pair = inner.find(|p| p.as_rule() == Rule::expr).unwrap();
            let expr = build_expr(expr_pair);
            Stmt::Print(expr)
        }
        Rule::let_stmt => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let expr = build_expr(inner.next().unwrap());
            Stmt::Let(name, expr)
        }
        Rule::if_stmt => {
            let mut inner = pair.into_inner();
            let condition = build_expr(inner.next().unwrap());
            let then_block = build_block(inner.next().unwrap());
            let else_block = inner.next().map(build_block);
            Stmt::If {
                condition,
                then_branch: then_block,
                else_branch: else_block,
            }
        }
        Rule::while_stmt => {
            let mut inner = pair.into_inner();
            let condition = build_expr(inner.next().unwrap());
            let body = build_block(inner.next().unwrap());
            Stmt::While { condition, body }
        }
        Rule::break_stmt => Stmt::Break,
        Rule::continue_stmt => Stmt::Continue,
        Rule::fn_stmt => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let param_list = inner.next().unwrap();
            let params = param_list
                .into_inner()
                .map(|p| p.as_str().to_string())
                .collect();
            let body = build_block(inner.next().unwrap());
            Stmt::Fn { name, params, body }
        }
        Rule::call_stmt => {
            let expr = build_expr(pair.into_inner().next().unwrap());
            if let Expr::Call(name, args) = expr {
                Stmt::Call(name, args)
            } else {
                panic!("Expected call expression in call_stmt");
            }
        }
        Rule::return_stmt => {
            let expr = build_expr(pair.into_inner().next().unwrap());
            Stmt::Return(expr)
        }
        _ => unreachable!(),
    }
}

/// Builds an expression from its Pest pair representation.
///
/// Handles literals, variables, binary operations, comparisons, and function calls.
///
/// # Arguments
/// * `pair` - A Pest pair representing an expression.
///
/// # Returns
/// An `Expr` enum variant representing the parsed expression.
fn build_expr(pair: pest::iterators::Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expr => {
            let mut inner = pair.into_inner();
            let mut left = build_expr(inner.next().unwrap());
            while let Some(op_pair) = inner.next() {
                let right = build_expr(inner.next().unwrap());
                let op = match op_pair.as_str() {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    _ => unreachable!(),
                };
                left = Expr::Binary(Box::new(left), op, Box::new(right));
            }
            left
        }
        Rule::term => {
            let mut inner = pair.into_inner();
            let mut left = build_expr(inner.next().unwrap());
            while let Some(op_pair) = inner.next() {
                let op = match op_pair.as_str() {
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => panic!("Unexpected operator in term: {:?}", op_pair.as_str()),
                };
                let right = build_expr(inner.next().unwrap());
                left = Expr::Binary(Box::new(left), op, Box::new(right));
            }
            left
        }
        Rule::factor => build_expr(pair.into_inner().next().unwrap()),
        Rule::number => Expr::Int(pair.as_str().parse().unwrap()),
        Rule::string => {
            let s = pair.as_str();
            Expr::Str(s[1..s.len() - 1].to_string()) // remove quotes
        }
        Rule::ident => Expr::Var(pair.as_str().to_string()),
        Rule::comparison => {
            let mut inner = pair.into_inner();
            let left = build_expr(inner.next().unwrap());
            if let Some(op_pair) = inner.next() {
                let right = build_expr(inner.next().unwrap());
                let op = match op_pair.as_str() {
                    ">" => Op::Greater,
                    "<" => Op::Less,
                    ">=" => Op::GreaterEq,
                    "<=" => Op::LessEq,
                    "==" => Op::Equal,
                    "!=" => Op::NotEqual,
                    _ => unreachable!(),
                };
                Expr::Binary(Box::new(left), op, Box::new(right))
            } else {
                left
            }
        }
        Rule::call_expr => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let args = if let Some(arg_list) = inner.next() {
                arg_list.into_inner().map(build_expr).collect()
            } else {
                Vec::new()
            };
            Expr::Call(name, args)
        }
        Rule::primary => build_expr(pair.into_inner().next().unwrap()),
        _ => unreachable!(),
    }
}

/// Builds a block of statements from a Pest pair.
///
/// Delegates to `build_ast` to convert the inner pairs into a vector of statements.
///
/// # Arguments
/// * `pair` - A Pest pair representing a block.
///
/// # Returns
/// A vector of `Stmt` representing the block's contents.
fn build_block(pair: pest::iterators::Pair<Rule>) -> Vec<Stmt> {
    build_ast(pair.into_inner())
}
