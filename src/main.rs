use std::{collections::HashMap, env::Vars};

#[derive(Debug)]
enum Expr {
    Number(f64), // 2.0 or 3.4
    Variable(String), // x or y
    Add(Box<Expr>, Box<Expr>), // x + 2.3 or 2.4 + 2.1
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>)
}


fn evaluate(expr: &Expr, vars: &HashMap<String, f64>) -> f64 {
    match expr {
        Expr::Number(n) => *n, // just return the number
        Expr::Variable(name) => {
            *vars.get(name).expect("variable not found") // Search for the variable in the map else panic
        },
        Expr::Add(left, right) => evaluate(left, vars) + evaluate(right, vars),
        Expr::Sub(left, right) => evaluate(left, vars) - evaluate(right, vars),
        Expr::Mul(left, right) => evaluate(left, vars) * evaluate(right, vars),
        Expr::Div(left, right) => evaluate(left, vars) / evaluate(right, vars),
    }
}

// Pretty print => (x + 2)
fn to_string(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Variable(name) => name.clone(),
        Expr::Add(left, right) => format!("({} + {})", to_string(left), to_string(right)),
        Expr::Sub(left, right) => format!("({} - {})", to_string(left), to_string(right)),
        Expr::Mul(left, right) => format!("({} * {})", to_string(left), to_string(right)),
        Expr::Div(left, right) => format!("({} / {})", to_string(left), to_string(right)),
    }
}


fn main() {
    // Build an expression like x + 2
    let expr: Expr = Expr::Add(
        Box::new(Expr::Variable("x".to_string())),
        Box::new(Expr::Number(2.0)),
    ); // left = x, right = 2.0

    // Define the value of variable x
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 3.0); // x = 3.0
    let result = evaluate(&expr, &vars);

    println!("Expression: {}", to_string(&expr)); // prints (x + 2)
    println!("Result: {}", result); // prints 5.0

}
