use crate::expr::{to_string, Expr};

pub fn differentiate(expr: &Expr, var: &str) -> Expr {
    match expr {
        // derivative of a constant = 0
        Expr::Number(_) => Expr::Number(0.0),
        
        // derivative of x wrt x is 1 else 0
        Expr::Variable(name) => {
            if name == var {
                Expr::Number(1.0)
            } else {
               Expr::Number(0.0)
            }
        },

        // (f + g)' = f' + g'
        Expr::Add(left, right) => Expr::Add(
            Box::new(differentiate(left, var)),
            Box::new(differentiate(right, var))
        ),

        // (f - g)' = f' - g'
        Expr::Sub(left, right) => Expr::Sub(
            Box::new(differentiate(&left, var)),
            Box::new(differentiate(&right, var))
        ),

        // (f * g)' = f'g + fg'
        Expr::Mul(left, right) => Expr::Add(
            Box::new(Expr::Mul(
                Box::new(differentiate(&left, var)),
                right.clone(),
            )),
            Box::new(Expr::Mul(
                left.clone(),
                Box::new(differentiate(&right, var)),
            )),
        ),


        Expr::Div(_left, _right) => {
            Expr::Number(1.0)
        },
        Expr::Pow(_base, _power ) => {
            Expr::Number(2.0)
        },
        Expr::Func(_name, _args) => {
            Expr::Number(3.0)
        },
        _ => panic!("Unknown Function: {}", to_string(&expr)),
    }
}
