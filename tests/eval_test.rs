use std::collections::HashMap;
use proton_lite::expr::*;
use proton_lite::eval::evaluate;

#[test]
fn test_addition() {
    let expr = Expr::Add(
        Box::new(Expr::Number(2.0)),
        Box::new(Expr::Number(3.0)),
    );
    let vars = HashMap::new();
    let result = evaluate(&expr, &vars);

    assert_eq!(result, 5.0);
}
