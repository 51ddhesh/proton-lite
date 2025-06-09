use crate::expr::{to_string, Expr};


#[allow(unreachable_patterns)]
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

        // (f / g)' = (f'g - fg')/g^2
        Expr::Div(left, right) => Expr::Div(
            Box::new(
                Expr::Sub(
                    Box::new(
                        Expr::Mul(
                            Box::new(differentiate(&left, var)),
                            right.clone(),
                        )
                    ),
                    Box::new(
                        Expr::Mul(
                            left.clone(),
                            Box::new(differentiate(&right, var)),
                        ),
                    ),
                )
            ),
            Box::new(
                Expr::Pow(
                    right.clone(),
                    Box::new(
                        Expr::Number(2.0),
                    ),
                )
            ),
        ),

        Expr::Pow(base, exponent ) => {
            match (&** base, &** exponent) { 
                // (f ^ n)' = n * f ^ (n - 1) * f'
                (_, Expr::Number(n)) => Expr::Mul(
                    Box::new(
                        Expr::Mul(
                            Box::new(Expr::Number(*n)),
                            Box::new(
                                Expr::Pow(
                                    base.clone(),
                                    Box::new(Expr::Number(*n - 1.0)),
                                )
                            )
                        )
                    ),
                    Box::new(differentiate(&base, var)),
                ),
                // General case: (f ^ g)' = f^g * (g' * ln(f) + g * f'/f)
                _ => {
                    let f = base.clone();
                    let g = exponent.clone();
                    let f_prime = differentiate(&f, var);
                    let g_prime = differentiate(&g, var);
                    let ln_f = Expr::Func(
                        "ln".to_string(),
                        vec![*f.clone()],
                    );
                    let g_prime_ln_f = Expr::Mul(Box::new(g_prime), Box::new(ln_f));
                    let f_prime_upon_f = Expr::Div(Box::new(f_prime), f.clone());
                    let g_fprime_upon_f = Expr::Mul(g.clone(), Box::new(f_prime_upon_f));
                    let product = Expr::Mul(Box::new(g_prime_ln_f), Box::new(g_fprime_upon_f));
                    Expr::Mul(
                        Box::new(Expr::Pow(f, g)),
                        Box::new(product),
                    )
                }
            }
        },
        
        Expr::Func(name, args) => {
            if args.len() != 1 {
                panic!("Differentiation only supports single-argument functions currently");
            }

            let arg = &args[0];
            let d_arg = differentiate(arg, var);
            
            match name.as_str() {
                // (sin(x))' = cos(x)
                "sin" => Expr::Mul(
                    Box::new(
                        Expr::Func(
                            "cos".to_string(),
                            vec![arg.clone()]
                        )
                    ),
                    Box::new(d_arg),
                ),

                // (cos(x))' = -sin(x)
                "cos" => Expr::Mul(
                    Box::new(
                        Expr::Mul(
                            Box::new(Expr::Number(-1.0)),
                            Box::new(
                                Expr::Func(
                                    "sin".to_string(),
                                    vec![arg.clone()]
                                )
                            )
                        )
                    ),
                Box::new(d_arg)
                ),
                

                // (tan(x))' = sec^2(x) = 1 / cos^2(x)
                "tan" => Expr::Mul(
                    Box::new(Expr::Div(
                        Box::new(Expr::Number(1.0)),
                        Box::new(Expr::Pow(Box::new(Expr::Func("cos".into(), vec![arg.clone()])), Box::new(Expr::Number(2.0))))
                    )),
                    Box::new(d_arg)
                ),

                // (ln(x))' = 1 / x
                "ln" => Expr::Div(
                    Box::new(d_arg), 
                    Box::new(arg.clone())
                ),

                // (sqrt(x))' = 1/(2 * sqrt(x))
                "sqrt" => Expr::Div(
                    Box::new(d_arg),
                    Box::new(
                        Expr::Mul(
                            Box::new(Expr::Number(2.0)),
                            Box::new(Expr::Func("sqrt".into(), vec![arg.clone()])),
                        )
                    )
                ),

                _ => {
                    panic!("Unsupported Function: {}", name);
                }
            }

        },
        _ => panic!("Unknown Function: {}", to_string(&expr)),
    }
}
