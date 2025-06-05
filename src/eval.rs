use std::collections::HashMap;

use crate::expr::Expr;

pub fn evaluate(expr: &Expr, vars: &HashMap<String, f64>) -> f64 {
    match expr {
        Expr::Number(n) => *n, // just return the number itself
        Expr::Variable(name) => { // Search for the variable in the map else panic
            *vars.get(name)
                .unwrap_or_else(|| {
                    println!("Warning: variable {} not found", name);
                    &f64::NAN
                }) 
        },
        Expr::Add(left, right) => evaluate(left, vars) + evaluate(right, vars),
        Expr::Sub(left, right) => evaluate(left, vars) - evaluate(right, vars),
        Expr::Mul(left, right) => evaluate(left, vars) * evaluate(right, vars),
        Expr::Div(left, right) => evaluate(left, vars) / evaluate(right, vars),
        Expr::Pow(left, right) => evaluate(left, vars).powf(evaluate(right, vars)),
        Expr::Func(name, args) => {
            let values: Vec<f64> = args.iter().map(|arg| evaluate(arg, vars)).collect();
            match name.as_str() {
                "sin" => {
                    if values.len() != 1 {
                        panic!("sin() expects 1 argument");
                    }
                    values[0].sin()
                },
                "cos" => {
                    if values.len() != 1 {
                        panic!("cos() expects 1 argument");
                    }
                    values[0].cos()
                },
                "tan" => {
                    if values.len() != 1 {
                        panic!("tan() expects 1 argument");
                    }
                    values[0].tan()
                },
                "ln" => {
                    if values.len() != 1 {
                        panic!("ln() expects 1 argument");
                    }
                    values[0].ln()
                },
                "log10" => {
                    if values.len() != 1 {
                        panic!("log10() expects 1 argument");
                    }
                    values[0].log10()
                },
                "sqrt" => {
                    if values.len() != 1 {
                        panic!("sqrt() expects 1 argument");
                    }
                    values[0].sqrt()
                },
                "max" => values.iter().cloned()
                    .fold(std::f64::NEG_INFINITY, f64::max),
                
                "min" => values.iter().cloned()
                    .fold(std::f64::INFINITY, f64::min),

                _ => panic!("Unknown Function: {}", name),
            }
        },
    }
}
