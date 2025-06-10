use crate::expr::{Expr};

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
        

        Expr::Mul(left, right) => {
            match (&** left, &** right) {
                // ∫kf = k∫f
                (Expr::Number(c), f) | (f, Expr::Number(c)) => {
                    Expr::Mul(
                        Box::new(Expr::Number(*c)),
                        Box::new(integrate(&f, var)),
                    )
                },
                _ => panic!("Yet to implement Integration by Parts"),
                // TODO: Add integration by parts     
            }
        }

        Expr::Pow(base, exponent) if matches!(**base, Expr::Variable(ref v) if v == var) => {
            if let Expr::Number(n) = &**exponent {
                // ∫1/x = ln(x)
                if *n == -1.0 {
                    Expr::Func(
                        "ln".to_string(),
                        vec![Expr::Variable(var.to_string())],
                    )
                } else {
                    // ∫x^n = x^(n + 1) / (n + 1)
                    Expr::Div(
                        Box::new(
                            Expr::Pow(
                                Box::new(Expr::Variable(var.to_string())),
                                Box::new(Expr::Number(n + 1.0))
                            )
                        ),
                        Box::new(Expr::Number(n + 1.0))
                    )
                }
            } else {
                panic!("Non-numeric exponent not supported yet")
            }
        }

        Expr::Func(name, args) if args.len() == 1 => {
            let arg = &args[0];
            match name.as_str() {
                "sin" => Expr::Mul(
                    Box::new(Expr::Number(-1.0)),
                    Box::new(Expr::Func("cos".into(), vec![arg.clone()]))
                ),

                "cos" => Expr::Func(
                    "sin".to_string(),
                    vec![arg.clone()]
                ),

                _ => todo!()
            }
        }

        _ => todo!()

    }
}
