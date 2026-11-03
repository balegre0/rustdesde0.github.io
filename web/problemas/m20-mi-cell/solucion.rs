use std::cell::UnsafeCell;

struct MiCell<T: Copy> {
    valor: UnsafeCell<T>,
}

impl<T: Copy> MiCell<T> {
    fn new(v: T) -> Self {
        MiCell { valor: UnsafeCell::new(v) }
    }
    fn get(&self) -> T {
        // SAFETY: MiCell es !Sync (no hay otros hilos) y nunca prestamos el interior.
        unsafe { *self.valor.get() }
    }
    fn set(&self, v: T) {
        // SAFETY: igual que en get: nadie más observa el interior ahora.
        unsafe { *self.valor.get() = v }
    }
    fn reemplazar(&self, v: T) -> T {
        // SAFETY: igual que en get; el &mut temporal muere al acabar la línea.
        unsafe { std::mem::replace(&mut *self.valor.get(), v) }
    }
}
