#[derive(Debug)]
enum Expr {
    Number(f64), // 2.0 or 3.4
    Variable(String), // x or y
    Add(Box<Expr>, Box<Expr>) // x + 2.3 or 2.4 + 2.1
}

fn main() {
    
    // Create an expression like x + 2.5
    let expr: Expr = Expr::Add(
        Box::new(Expr::Variable("x".to_string())),
        Box::new(Expr::Number(2.5))
    );

    println!("{:?}", expr); // Outputs Add(Variable("x"), Number(2.5))
}
