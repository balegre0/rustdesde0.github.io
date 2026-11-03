use std::fmt::Write;

fn main() {
    // 1. write! es como print!, pero escribe en lo que tú le digas
    let mut informe = String::new();
    for n in 1..=3 {
        write!(informe, "{}-", n).unwrap();
    }
    writeln!(informe, "fin").unwrap();
    print!("{}", informe);

    // 2. format! crea un String nuevo con el mismo mini-lenguaje
    let s = format!("{:>6}|{:<6}|{:^6}|", "der", "izq", "cen");
    println!("{}", s);
    let pi = 3.14159;
    println!("{pi:.2} {pi:08.3}");
}
