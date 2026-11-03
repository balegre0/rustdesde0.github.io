use std::cell::UnsafeCell;

struct MiCell<T: Copy> {
    valor: UnsafeCell<T>,
}

impl<T: Copy> MiCell<T> {
    fn new(v: T) -> Self {
        MiCell { valor: UnsafeCell::new(v) }
    }
    fn get(&self) -> T {
        todo!()
    }
    fn set(&self, v: T) {
        todo!()
    }
    fn reemplazar(&self, v: T) -> T {
        todo!()
    }
}
