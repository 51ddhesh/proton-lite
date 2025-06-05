use std::io::{self, Write};

fn repl() {
    let mut input = String::new();
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            break;
        }
        
        // TODO: parse input into `Expr`

        println!("You entered: {}", input.trim());
    }
}
