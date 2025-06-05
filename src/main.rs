use std::{collections::HashMap};

use proton_lite::eval::evaluate;
use proton_lite::expr::{Expr, to_string};

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


    // for arguments of Expr::Func(), instead of using Box<>, use Vec<>
    expr = Expr::Func(
        "sin".to_string(),
        vec![Expr::Variable("θ".to_string())],
    );

    vars.insert("θ".to_string(), std::f64::consts::FRAC_PI_2); // pi / 2 == 90 degrees
    result = evaluate(&expr, &vars);

    println!("Expression: {}, θ = {:?} (π/2)", to_string(&expr), vars.get("θ"));
    println!("Result: {}", result);

    // Test ln(e)
    expr = Expr::Func(
        "ln".to_string(),
        vec![Expr::Variable("e".to_string())],
    );
    vars.insert("e".to_string(), std::f64::consts::E);
    result = evaluate(&expr, &vars);

    println!("Expression: {}", to_string(&expr));
    println!("Result: {}", result);

    expr = Expr::Func(
        "sin".to_string(),
        vec![Expr::Func(
            "ln".to_string(),
            vec![Expr::Variable("w".to_string())],
        )],
    );
    vars.insert("w".to_string(), 1.0);
    result = evaluate(&expr, &vars);

    println!("Expression: {}, w = {:?}", to_string(&expr), vars.get("w"));
    println!("Result: {}", result);


    expr = Expr::Pow(
        Box::new(Expr::Number(std::f64::consts::E)),
        Box::new(Expr::Func(
            "sin".to_string(),
            vec![Expr::Variable("x".to_string())],
        )),
    );

    vars.insert("x".to_string(), std::f64::consts::FRAC_PI_2);
    result = evaluate(&expr, &vars);

    println!("Expression: {}", to_string(&expr));
    println!("Result: {}", result);

    // Test max(a, b)
    expr = Expr::Func(
        "max".to_string(),
        vec![
            Expr::Variable("a".to_string()),
            Expr::Variable("b".to_string()),
        ],
    );
    vars.insert("a".to_string(), 5.0);
    vars.insert("b".to_string(), 7.0);

    result = evaluate(&expr, &vars);

    println!("Expression: {}, a = {:?}, b = {:?}", to_string(&expr), vars.get("a"), vars.get("b"));
    println!("Result: {}", result);


    // Test min(a, b, c)
    expr = Expr::Func(
        "min".to_string(),
        vec![
            Expr::Variable("a".to_string()),
            Expr::Variable("b".to_string()),
            Expr::Variable("c".to_string()),
        ],
    );
    vars.insert("a".to_string(), 1.0);
    vars.insert("b".to_string(), 2.0);
    vars.insert("c".to_string(), 3.0);

    result = evaluate(&expr, &vars);

    println!("Expression: {}, a = {:?}, b = {:?}, c = {:?}", to_string(&expr), 
        vars.get("a"),
        vars.get("b"),
        vars.get("c")
    );
    println!("Result: {}", result);

}


fn main() {
    test_operations();
}
