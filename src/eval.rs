use crate::ast::{Expr, Stmt, Context, LoopControl, Op};

/// Evaluates an expression within the given context and returns its result as a string.
///
/// Supports literals, variables, binary operations, and function calls.
/// Binary operations are evaluated as integer arithmetic or comparisons.
/// Function calls are executed with a new local context.
///
/// # Arguments
/// * `expr` - The expression to evaluate.
/// * `ctx` - The current execution context containing variables and functions.
///
/// # Returns
/// A string representing the result of the evaluated expression.
pub(crate) fn eval_expr(expr: &Expr, ctx: &Context) -> String {
    match expr {
        Expr::Int(i) => i.to_string(),
        Expr::Str(s) => s.clone(),
        Expr::Var(name) => ctx
            .variables
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("Undefined variable: {}", name)),
        Expr::Binary(left, op, right) => {
            let l = eval_expr(left, ctx).parse::<i64>().unwrap();
            let r = eval_expr(right, ctx).parse::<i64>().unwrap();
            let result = match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => l / r,
                Op::Greater => (l > r) as i64,
                Op::Less => (l < r) as i64,
                Op::GreaterEq => (l >= r) as i64,
                Op::LessEq => (l <= r) as i64,
                Op::Equal => (l == r) as i64,
                Op::NotEqual => (l != r) as i64,
            };
            result.to_string()
        }
        Expr::Call(name, args) => {
            let (params, body) = ctx
                .functions
                .get(name)
                .unwrap_or_else(|| panic!("Undefined function: {}", name))
                .clone();

            if params.len() != args.len() {
                panic!(
                    "Function '{}' expected {} args, got {}",
                    name,
                    params.len(),
                    args.len()
                );
            }

            let mut local_ctx = Context::default();
            for (param, arg) in params.iter().zip(args.iter()) {
                let value = eval_expr(arg, ctx);
                local_ctx.variables.insert(param.clone(), value);
            }

            for stmt in body {
                match exec_stmt(&stmt, &mut local_ctx) {
                    LoopControl::Return(val) => return val,
                    LoopControl::None => continue,
                    _ => panic!("Unexpected control flow in function"),
                }
            }
            "".to_string()
        }
    }
}

/// Executes a single statement within the given mutable context.
///
/// Handles all statement types including variable assignment, control flow,
/// function definitions, function calls, and return statements.
///
/// # Arguments
/// * `stmt` - The statement to execute.
/// * `ctx` - The mutable execution context.
///
/// # Returns
/// A `LoopControl` value indicating control flow status (e.g., break, continue, return).
pub(crate) fn exec_stmt(stmt: &Stmt, ctx: &mut Context) -> LoopControl {
    match stmt {
        Stmt::Print(expr) => {
            println!("{}", eval_expr(expr, ctx));
            LoopControl::None
        }
        Stmt::Let(name, expr) => {
            let value = eval_expr(expr, ctx);
            ctx.variables.insert(name.clone(), value);
            LoopControl::None
        }
        Stmt::Break => LoopControl::Break,
        Stmt::Continue => LoopControl::Continue,
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let cond_value = eval_expr(condition, ctx);
            let is_true = cond_value != "0" && cond_value != "" && cond_value != "false";
            let fallback = Vec::new();
            let branch = if is_true {
                then_branch
            } else {
                else_branch.as_ref().unwrap_or(&fallback)
            };
            for stmt in branch {
                match exec_stmt(stmt, ctx) {
                    LoopControl::None => continue,
                    control => return control,
                }
            }
            LoopControl::None
        }
        Stmt::While { condition, body } => {
            while eval_expr(condition, ctx) != "0" {
                for stmt in body {
                    match exec_stmt(stmt, ctx) {
                        LoopControl::None => continue,
                        LoopControl::Break => return LoopControl::None,
                        LoopControl::Continue => break,
                        LoopControl::Return(val) => return LoopControl::Return(val),
                    }
                }
            }
            LoopControl::None
        }
        Stmt::Fn { name, params, body } => {
            ctx.functions
                .insert(name.clone(), (params.clone(), body.to_vec()));
            LoopControl::None
        }
        Stmt::Call(name, args) => {
            let (params, body) = ctx.functions.get(name).unwrap().clone();
            let mut local_ctx = Context::default();
            for (param, arg) in params.iter().zip(args.iter()) {
                let value = eval_expr(arg, ctx);
                local_ctx.variables.insert(param.clone(), value);
            }
            for stmt in body {
                exec_stmt(&stmt, &mut local_ctx);
            }
            LoopControl::None
        }
        Stmt::Return(expr) => {
            let value = eval_expr(expr, ctx);
            LoopControl::Return(value)
        }
    }
}
