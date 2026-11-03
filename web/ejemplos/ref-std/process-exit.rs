use std::process;

fn dividir(a: i32, b: i32) -> i32 {
    if b == 0 {
        eprintln!("error: división por cero");
        process::exit(2); // termina YA, con código 2
    }
    a / b
}

fn main() {
    println!("{}", dividir(10, 2));
    println!("{}", dividir(1, 0));
    println!("esto no se imprime");
}
