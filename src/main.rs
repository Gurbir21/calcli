use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let err = match args.len() {
        1 => Some("No Operation or Operands"),
        2 => Some("Operation with no Operands, this operation requires 2 operands"),
        3 => Some("1 operand found, Operation requires 2 operands"),
        _ => None,
    };

    if let Some(msg) = err {
        println!("{msg}");
        return;
    }

    let num_1: f64 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a Number!");
            return;
        }
    };
    
    let num_2: f64 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a Number!");
            return;
        } 
    };

    let res : f64 = match args[1].as_str() {
        "add" => add(num_1, num_2),
        "sub" => subtract(num_1, num_2),
        "mul" => multiply(num_1, num_2),
        "div" => divide(num_1, num_2),
        "mod" => modulo(num_1, num_2),
        _ => {
            println!("There is no valid operation inputted");
            return;
        },
    };

    println!("{res}");


}
fn add(a:f64, b:f64) -> f64 {
    a + b
}

fn multiply(a:f64, b:f64) -> f64 {
    a * b
}

fn subtract(a:f64, b:f64) -> f64 {
    a - b
}

fn divide(a:f64, b:f64) -> f64 {
    a / b
    
}

fn modulo(a:f64, b:f64) -> f64 {
    a % b
}


