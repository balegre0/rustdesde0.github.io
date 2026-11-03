// Mutabilidad interior bien hecha: todo pasa por UnsafeCell.
use std::cell::UnsafeCell;

struct MiCell<T: Copy> {
    valor: UnsafeCell<T>,
}

impl<T: Copy> MiCell<T> {
    fn new(v: T) -> Self {
        MiCell { valor: UnsafeCell::new(v) }
    }
    fn get(&self) -> T {
        // SAFETY: MiCell no es Sync (UnsafeCell no lo es), así que no hay otros
        // hilos; y nunca damos referencias al interior, solo copias.
        unsafe { *self.valor.get() }
    }
    fn set(&self, v: T) {
        // SAFETY: mismo razonamiento: nadie más tiene un préstamo del interior.
        unsafe { *self.valor.get() = v }
    }
}

fn main() {
    let c = MiCell::new(1);
    let a = &c; // 1. dos préstamos COMPARTIDOS...
    let b = &c;
    a.set(10); // 2. ...y aun así mutamos
    b.set(b.get() + 5);
    println!("{}", c.get());
    println!("size_of MiCell<i32> = {}", size_of::<MiCell<i32>>());
}
