#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64), // 2.0 or 3.4
    Variable(String), // x or y
    Add(Box<Expr>, Box<Expr>), // x + 2.3 or 2.4 + 2.1
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Func(String, Vec<Expr>), // For functions like sin(x) and ln(x)
}


// Pretty print => (x + 2)
pub fn to_string(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Variable(name) => name.clone(),
        Expr::Add(left, right) => format!("({} + {})", to_string(left), to_string(right)),
        Expr::Sub(left, right) => format!("({} - {})", to_string(left), to_string(right)),
        Expr::Mul(left, right) => format!("({} * {})", to_string(left), to_string(right)),
        Expr::Div(left, right) => format!("({} / {})", to_string(left), to_string(right)),
        Expr::Pow(left, right) => format!("({} ^ {})", to_string(left), to_string(right)),
        Expr::Func(name, args) => {
            let arg_str: Vec<String> = args.iter().map(to_string).collect();
            format!("{}({})", name, arg_str.join(", "))
        }
    }
}
