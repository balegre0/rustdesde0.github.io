use std::cell::{Cell, RefCell};

thread_local! {
    // Cada hilo tiene SU PROPIA copia de estas variables.
    static CONTADOR: Cell<u32> = const { Cell::new(0) };
    static REGISTRO: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

fn anotar(texto: &str) {
    CONTADOR.set(CONTADOR.get() + 1);
    REGISTRO.with_borrow_mut(|r| r.push(texto.to_string()));
}

fn main() {
    anotar("inicio");
    anotar("trabajo");
    println!("{} anotaciones", CONTADOR.get());
    REGISTRO.with_borrow(|r| println!("{r:?}"));
}
