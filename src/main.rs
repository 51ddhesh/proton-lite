use std::{collections::HashMap};

#[derive(Debug)]
enum Expr {
    Number(f64), // 2.0 or 3.4
    Variable(String), // x or y
    Add(Box<Expr>, Box<Expr>), // x + 2.3 or 2.4 + 2.1
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Func(String, Box<Expr>), // For functions like sin(x) and ln(x)
}


fn evaluate(expr: &Expr, vars: &HashMap<String, f64>) -> f64 {
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
        Expr::Func(name, arg) => {
            let val: f64 = evaluate(arg, vars);
            match name.as_str() { // Rust internal functions are used
                "sin" => val.sin(),
                "cos" => val.cos(),
                "tan" => val.tan(),
                "ln" => val.ln(),
                "log10" => val.log10(),
                "sqrt" => val.sqrt(),
                _ => panic!("Unknown function: {}", name),
            }
        },
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
        Expr::Pow(left, right) => format!("({} ^ {})", to_string(left), to_string(right)),
        Expr::Func(name, arg) => format!("{}({})", name, to_string(arg)),
    }
}


fn test_operations() {
    // Build an expression like x + 2
    let mut expr: Expr = Expr::Add(
        Box::new(Expr::Variable("x".to_string())),
        Box::new(Expr::Number(2.0)),
    ); // left = x, right = 2.0

    // Define the value of variable x
    let mut vars: HashMap<String, f64> = HashMap::new();
    vars.insert("x".to_string(), 3.0); // x = 3.0
    let mut result: f64 = evaluate(&expr, &vars);

    println!("Expression: {}, x = {:?}", to_string(&expr), vars.get("x")); // prints (x + 2)
    println!("Result: {}", result); // prints 5.0

    expr = Expr::Sub(
        Box::new(Expr::Variable("y".to_string())),
        Box::new(Expr::Number(5.0)),
    );

    vars.insert("y".to_string(), 10.0);
    result = evaluate(&expr, &vars);

    println!("Expression: {}, y = {:?}", to_string(&expr), vars.get("y")); // prints (y - 5)
    println!("Result: {}", result); // prints 5

    expr = Expr::Mul(
        Box::new(Expr::Variable("z".to_string())),
        Box::new(Expr::Number(2.0)),
    );

    vars.insert("z".to_string(), 5.0);
    result = evaluate(&expr, &vars);

    println!("Expression: {}, z = {:?}", to_string(&expr), vars.get("z")); // prints (z * 2)
    println!("Result: {}", result); // prints 10


    expr = Expr::Div(
        Box::new(Expr::Variable("t".to_string())),
        Box::new(Expr::Number(3.0)),
    );

    vars.insert("t".to_string(), 9.0);
    result = evaluate(&expr, &vars);

    println!("Expression: {}, t = {:?}", to_string(&expr), vars.get("t")); // prints (t / 3)
    println!("Result: {}", result); // prints 3


    expr = Expr::Pow(
        Box::new(Expr::Variable("u".to_string())),
        Box::new(Expr::Variable("v".to_string())),
    );

    vars.insert("u".to_string(), 2.0);
    vars.insert("v".to_string(), 3.0);
    result = evaluate(&expr, &vars);

    println!("Expression: {}, u = {:?}, v = {:?}", to_string(&expr), vars.get("u"), vars.get("v")); // prints (u ^ v)
    println!("Result: {}", result); // prints 8


    expr = Expr::Func(
        "sin".to_string(),
        Box::new(Expr::Variable("θ".to_string())),
    );

    vars.insert("θ".to_string(), std::f64::consts::FRAC_PI_2); // pi / 2 == 90 degrees
    result = evaluate(&expr, &vars);

    println!("Expression: {}, θ = {:?} (π/2)", to_string(&expr), vars.get("θ"));
    println!("Result: {}", result);


    expr = Expr::Func(
        "ln".to_string(),
        Box::new(Expr::Variable("e".to_string())),
    );
    vars.insert("e".to_string(), std::f64::consts::E);
    result = evaluate(&expr, &vars);

    println!("Expression: {}", to_string(&expr));
    println!("Result: {}", result);

    expr = Expr::Func(
        "sin".to_string(),
        Box::new(Expr::Func(
            "ln".to_string(),
            Box::new(Expr::Variable("w".to_string())),
        )),
    );
    vars.insert("w".to_string(), 1.0);
    result = evaluate(&expr, &vars);

    println!("Expression: {}, w = {:?}", to_string(&expr), vars.get("w"));
    println!("Result: {}", result);


    expr = Expr::Pow(
        Box::new(Expr::Number(std::f64::consts::E)),
        Box::new(Expr::Func(
            "sin".to_string(),
            Box::new(Expr::Variable("x".to_string())),
        )),
    );

    vars.insert("x".to_string(), std::f64::consts::FRAC_PI_2);
    result = evaluate(&expr, &vars);

    println!("Expression: {}", to_string(&expr));
    println!("Result: {}", result);


}


fn main() {
    test_operations();
}
