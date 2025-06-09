use crate::expr::Expr;

pub fn integrate(expr: &Expr, var: &str) -> Expr {
    match expr {
        // ∫c dx = cx + k
        Expr::Number(c) => {
            Expr::Mul(
                Box::new(Expr::Number(*c)),
                Box::new(Expr::Variable(var.to_string()))
            )
        },

        Expr::Variable(x) if x == var => {
            // ∫x dx = x^2 / 2
            Expr::Div(
                Box::new(
                    Expr::Pow(Box::new(Expr::Variable(var.to_string())), Box::new(Expr::Number(2.0)))
                ),
                Box::new(Expr::Number(2.0)),
            ) 
        },

        // ∫(f + g) = ∫f + ∫g
        Expr::Add(left, right) => {
            Expr::Add(
                Box::new(integrate(&left, var)),
                Box::new(integrate(&right, var))
            )
        },

        // ∫(f - g) = ∫f - ∫g
        Expr::Sub(left, right) => {
            Expr::Sub(
                Box::new(integrate(&left, var)),
                Box::new(integrate(&right, var))
            )
        },

        // TODO: Add integration by parts     

        _ => todo!()

    }
}
