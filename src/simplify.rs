use crate::expr::Expr;

pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        // If the expression is a number or a variable, return it as it is
        Expr::Number(_) | Expr::Variable(_) => expr.clone(),

        // Recursive simplification for add
        Expr::Add(left, right) => {
            let left: Expr = simplify(&left);
            let right: Expr = simplify(&right);

            match (&left, &right) {
                (Expr::Number(a), Expr::Number(b)) => Expr::Number(a + b),
                (Expr::Number(0.0), r) => r.clone(), // k + 0 = k
                (l, Expr::Number(0.0)) => l.clone(), // 0 + k = k
                _ => Expr::Add(Box::new(left), Box::new(right)), // Initiate a recursive call
            }
        },

        Expr::Sub(left, right) => {
            let left: Expr = simplify(&left);
            let right: Expr = simplify(&right);

            match (&left, &right) {
                (Expr::Number(a), Expr::Number(b)) => Expr::Number(a - b),
                (l, Expr::Number(0.0)) => l.clone(), // l - 0 = l
                _ => Expr::Sub(Box::new(left), Box::new(right)),
            }
        },

        Expr::Mul(left, right) => {
            let left: Expr = simplify(&left);
            let right: Expr = simplify(&right);

            match (&left, &right) {
                (Expr::Number(0.0), _) | (_, Expr::Number(0.0)) => Expr::Number(0.0), // 0 * l = r * 0 = 0
                (Expr::Number(1.0), r) => r.clone(), // 1 * x = x
                (l, Expr::Number(1.0)) => l.clone(), // x * 1 = x
                (Expr::Number(a), Expr::Number(b)) => Expr::Number(a * b),
                _ => Expr::Mul(Box::new(left), Box::new(right)),
            }
        },

        Expr::Div(left, right) => {
            let left: Expr = simplify(&left);
            let right: Expr = simplify(&right);
        
            match (&left, &right) {
                (_, Expr::Number(0.0)) => panic!("Cannot divide by zero"),
                (Expr::Number(0.0), _) => Expr::Number(0.0), // x / 0 - not possible
                (l, Expr::Number(1.0)) => l.clone(), // x / 1 = x
                (Expr::Number(a), Expr::Number(b)) => Expr::Number(a / b),
                _ => Expr::Div(Box::new(left), Box::new(right)),
            }
        },

        Expr::Pow(base, power) => {
            let base: Expr = simplify(&base);
            let power: Expr = simplify(&power);

            match (&base, &power) {
                (_, Expr::Number(0.0)) => Expr::Number(1.0),
                (l, Expr::Number(1.0)) => l.clone(),
                (Expr::Number(a), Expr::Number(b)) => Expr::Number(a.powf(*b)),
                _ => Expr::Pow(Box::new(base), Box::new(power)),
            }
        },

        Expr::Func(name, args) => {
            let simplified_args: Vec<Expr> = args.iter().map(|arg| simplify(arg)).collect();

            // Attempt constant folding for known unary functions
            match name.as_str() {
                "sin" => {
                    if let [Expr::Number(n)] = &simplified_args[..] {
                        return Expr::Number(n.sin());
                    }
                }
                "cos" => {
                    if let [Expr::Number(n)] = &simplified_args[..] {
                        return Expr::Number(n.cos());
                    }
                }
                "tan" => {
                    if let [Expr::Number(n)] = &simplified_args[..] {
                        return Expr::Number(n.tan());
                    }
                }
                "ln" => {
                    if let [Expr::Number(n)] = &simplified_args[..] {
                        return Expr::Number(n.ln());
                    }
                }
                "log10" => {
                    if let [Expr::Number(n)] = &simplified_args[..] {
                        return Expr::Number(n.log10());
                    }
                }
                "sqrt" => {
                    if let [Expr::Number(n)] = &simplified_args[..] {
                        return Expr::Number(n.sqrt());
                    }
                }
                "max" => {
                    if simplified_args.iter().all(|arg| matches!(arg, Expr::Number(_))) {
                        let max_val = simplified_args
                            .iter()
                            .filter_map(|arg| {
                                if let Expr::Number(n) = arg {
                                    Some(*n)
                                } else {
                                    None
                                }
                            })
                            .fold(f64::NEG_INFINITY, f64::max);
                        return Expr::Number(max_val);
                    }
                }
                "min" => {
                    if simplified_args.iter().all(|arg| matches!(arg, Expr::Number(_))) {
                        let min_val = simplified_args
                            .iter()
                            .filter_map(|arg| {
                                if let Expr::Number(n) = arg {
                                    Some(*n)
                                } else {
                                    None
                                }
                            })
                            .fold(f64::INFINITY, f64::min);
                        return Expr::Number(min_val);
                    }
                }
                _ => {} // fall through
            }

            Expr::Func(name.clone(), simplified_args)
        }
    }
}
