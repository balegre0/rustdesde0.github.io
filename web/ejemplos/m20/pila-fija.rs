// Una pila de capacidad fija sin asignar memoria: MaybeUninit + invariantes.
use std::mem::MaybeUninit;

/// Invariante: las casillas 0..len están inicializadas; len <= N.
struct PilaFija<T, const N: usize> {
    datos: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> PilaFija<T, N> {
    fn new() -> Self {
        PilaFija { datos: [const { MaybeUninit::uninit() }; N], len: 0 }
    }

    /// Si está llena, devuelve el valor dentro de Err.
    fn push(&mut self, v: T) -> Result<(), T> {
        if self.len == N {
            return Err(v);
        }
        self.datos[self.len].write(v);
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: la casilla len estaba inicializada; al bajar len ya no cuenta como viva.
        Some(unsafe { self.datos[self.len].assume_init_read() })
    }
}

impl<T, const N: usize> Drop for PilaFija<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

fn main() {
    let mut p: PilaFija<String, 2> = PilaFija::new();
    println!("{:?}", p.push("a".to_string()));
    println!("{:?}", p.push("b".to_string()));
    println!("{:?}", p.push("c".to_string()));
    println!("{:?}", p.pop());
    println!("{:?}", p.push("d".to_string()));
    println!("{:?} {:?} {:?}", p.pop(), p.pop(), p.pop());
}
