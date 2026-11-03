use std::cell::Cell;

// 1. Cada hilo tiene su propia copia de CONTADOR
thread_local! {
    static CONTADOR: Cell<u32> = Cell::new(0);
}

fn incrementa() -> u32 {
    CONTADOR.with(|c| {
        c.set(c.get() + 1);
        c.get()
    })
}

fn main() {
    incrementa();
    incrementa();
    println!("en este hilo van {}", incrementa());
}
