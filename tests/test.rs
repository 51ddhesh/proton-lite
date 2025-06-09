use std::collections::HashMap;
use proton_lite::{expr::Expr};
use proton_lite::eval::evaluate;
use proton_lite::differentiate::differentiate;

#[test]
fn test_addition() {
    let expr = Expr::Add(
        Box::new(Expr::Number(2.0)),
        Box::new(Expr::Number(3.0)),
    );

    let vars: HashMap<String, f64> = HashMap::new();
    let mut result: f64 = evaluate(&expr, &vars);
    
    assert_eq!(result, 5.0);

    let expr = Expr::Add(
        Box::new(Expr::Number(-2.0)),
        Box::new(Expr::Number(3.0)),
    );
    
    result = evaluate(&expr, &vars);

    assert_eq!(result, 1.0);

}

#[test]
fn test_subtraction() {
    let expr: Expr = Expr::Sub(
        Box::new(Expr::Number(10.0)),
        Box::new(Expr::Number(5.0)),
    );

    let vars = HashMap::new();
    let result = evaluate(&expr, &vars);

    assert_eq!(result, 5.0);
}


#[test]
fn test_multiplication() {
    let expr: Expr = Expr::Mul(
        Box::new(Expr::Number(10.0)),
        Box::new(Expr::Number(5.0)),
    );

    let vars = HashMap::new();
    let result = evaluate(&expr, &vars);

    assert_eq!(result, 50.0);
}

#[test]
fn test_division() {
    let expr: Expr = Expr::Div(
        Box::new(Expr::Number(10.0)),
        Box::new(Expr::Number(5.0)),
    );

    let vars = HashMap::new();
    let result = evaluate(&expr, &vars);

    assert_eq!(result, 2.0);
}

#[test]
fn test_exponents() {
    let expr: Expr = Expr::Pow(
        Box::new(Expr::Number(2.0)),
        Box::new(Expr::Number(2.0)),
    );

    let vars: HashMap<String, f64> = HashMap::new();
    let result: f64 = evaluate(&expr, &vars);

    assert_eq!(result, 4.0);
}

#[test]
fn test_sin() {
    let expr: Expr = Expr::Func(
        "sin".to_string(),
        vec![
            Expr::Number(std::f64::consts::FRAC_PI_2)
        ],
    );

    let vars: HashMap<String, f64> = HashMap::new();
    let result: f64 = evaluate(&expr, &vars);

    assert_eq!(result, 1.0);
}

#[test]
fn test_variable() {
    let expr = Expr::Variable("x".to_string());
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 7.0);
    let result = evaluate(&expr, &vars);
    assert_eq!(result, 7.0);
}

#[test]
fn test_nested_expression() {
    let expr = Expr::Add(
        Box::new(Expr::Mul(
            Box::new(Expr::Number(2.0)),
            Box::new(Expr::Number(3.0)),
        )),
        Box::new(Expr::Number(4.0)),
    );
    let vars = HashMap::new();
    let result = evaluate(&expr, &vars);
    assert_eq!(result, 10.0);
}

#[test]
fn test_cos() {
    let expr = Expr::Func(
        "cos".to_string(),
        vec![Expr::Number(0.0)],
    );
    let vars = HashMap::new();
    let result = evaluate(&expr, &vars);
    assert_eq!(result, 1.0);
}

#[test]
fn test_multiple_variables() {
    let expr = Expr::Add(
        Box::new(Expr::Variable("a".to_string())),
        Box::new(Expr::Variable("b".to_string())),
    );
    let mut vars = HashMap::new();
    vars.insert("a".to_string(), 2.0);
    vars.insert("b".to_string(), 3.0);
    let result = evaluate(&expr, &vars);
    assert_eq!(result, 5.0);
}

#[test]
fn test_func_with_variable() {
    let expr = Expr::Func(
        "sin".to_string(),
        vec![Expr::Variable("theta".to_string())],
    );
    let mut vars = HashMap::new();
    vars.insert("theta".to_string(), std::f64::consts::FRAC_PI_2);
    let result = evaluate(&expr, &vars);
    assert_eq!(result, 1.0);
}

#[test]
fn test_zero_division() {
    let expr = Expr::Div(
        Box::new(Expr::Number(1.0)),
        Box::new(Expr::Number(0.0)),
    );
    let vars = HashMap::new();
    let result = evaluate(&expr, &vars);
    assert!(result.is_infinite());
}

#[test]
fn test_pow_zero_exponent() {
    let expr = Expr::Pow(
        Box::new(Expr::Number(5.0)),
        Box::new(Expr::Number(0.0)),
    );
    let vars = HashMap::new();
    let result = evaluate(&expr, &vars);
    assert_eq!(result, 1.0);
}

#[cfg(test)]
mod differentiate_tests {
    use super::*;
    use proton_lite::expr::Expr;

    #[test]
    fn test_constant_derivative() {
        let expr = Expr::Number(5.0);
        let d = differentiate(&expr, "x");
        assert_eq!(d, Expr::Number(0.0));
    }

    #[test]
    fn test_variable_derivative() {
        let expr = Expr::Variable("x".to_string());
        let d = differentiate(&expr, "x");
        assert_eq!(d, Expr::Number(1.0));
        let expr = Expr::Variable("y".to_string());
        let d = differentiate(&expr, "x");
        assert_eq!(d, Expr::Number(0.0));
    }

    #[test]
    fn test_addition_derivative() {
        let expr = Expr::Add(
            Box::new(Expr::Variable("x".to_string())),
            Box::new(Expr::Number(2.0)),
        );
        let d = differentiate(&expr, "x");
        assert_eq!(d, Expr::Add(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(0.0))));
    }

    #[test]
    fn test_multiplication_derivative() {
        let expr = Expr::Mul(
            Box::new(Expr::Variable("x".to_string())),
            Box::new(Expr::Variable("x".to_string())),
        );
        let d = differentiate(&expr, "x");
        // (x * x)' = x' * x + x * x' = 1*x + x*1 = x + x
        assert_eq!(d, Expr::Add(
            Box::new(Expr::Mul(Box::new(Expr::Number(1.0)), Box::new(Expr::Variable("x".to_string())))),
            Box::new(Expr::Mul(Box::new(Expr::Variable("x".to_string())), Box::new(Expr::Number(1.0))))
        ));
    }

    #[test]
    fn test_power_derivative_simple() {
        let expr = Expr::Pow(
            Box::new(Expr::Variable("x".to_string())),
            Box::new(Expr::Number(2.0)),
        );
        let d = differentiate(&expr, "x");
        // (x^2)' = 2 * x^(2-1) * 1 = 2 * x^1 * 1
        assert_eq!(d, Expr::Mul(
            Box::new(Expr::Mul(
                Box::new(Expr::Number(2.0)),
                Box::new(Expr::Pow(
                    Box::new(Expr::Variable("x".to_string())),
                    Box::new(Expr::Number(1.0)),
                ))
            )),
            Box::new(Expr::Number(1.0))
        ));
    }

    #[test]
    fn test_sin_derivative() {
        let expr = Expr::Func(
            "sin".to_string(),
            vec![Expr::Variable("x".to_string())],
        );
        let d = differentiate(&expr, "x");
        // (sin(x))' = cos(x) * 1
        assert_eq!(d, Expr::Mul(
            Box::new(Expr::Func("cos".to_string(), vec![Expr::Variable("x".to_string())])),
            Box::new(Expr::Number(1.0))
        ));
    }

    #[test]
    fn test_ln_derivative() {
        let expr = Expr::Func(
            "ln".to_string(),
            vec![Expr::Variable("x".to_string())],
        );
        let d = differentiate(&expr, "x");
        // (ln(x))' = 1 / x
        assert_eq!(d, Expr::Div(
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Variable("x".to_string()))
        ));
    }
}
