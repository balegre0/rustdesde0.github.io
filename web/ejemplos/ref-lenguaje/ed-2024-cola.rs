use std::cell::RefCell;

fn largo() -> usize {
    let c = RefCell::new(String::from("rust"));
    // Edición 2024: el temporal `c.borrow()` de la expresión final
    // se destruye ANTES que la variable `c`. En 2021 esto no compilaba (E0597).
    c.borrow().len()
}

fn main() {
    println!("{}", largo());
}
