use std::cell::{Cell, UnsafeCell};

// Un asignador «bump» (de avance): reserva avanzando un puntero sobre una zona fija.
// No libera piezas sueltas: solo todo de golpe. Es el primero que se escribe en un kernel.
struct Bump {
    memoria: UnsafeCell<[u8; 64]>,
    siguiente: Cell<usize>,
}

impl Bump {
    fn new() -> Bump {
        Bump { memoria: UnsafeCell::new([0; 64]), siguiente: Cell::new(0) }
    }

    /// Reserva `tam` bytes alineados a `align` (potencia de 2). None si no queda sitio.
    fn reservar(&self, tam: usize, align: usize) -> Option<*mut u8> {
        let base = self.memoria.get() as *mut u8 as usize;
        let inicio = (base + self.siguiente.get()).next_multiple_of(align) - base;
        let fin = inicio.checked_add(tam)?;
        if fin > 64 {
            return None;
        }
        self.siguiente.set(fin);
        // SAFETY: inicio + tam <= 64, así que el puntero queda dentro del array.
        Some(unsafe { (self.memoria.get() as *mut u8).add(inicio) })
    }

    fn reiniciar(&self) {
        self.siguiente.set(0);
    }
}

fn main() {
    let b = Bump::new();
    let base = b.memoria.get() as usize;
    for (tam, align) in [(3, 1), (8, 8), (4, 4), (40, 8), (16, 8)] {
        match b.reservar(tam, align) {
            Some(p) => println!("reservar({tam:2}, alinea {align}) → desplazamiento {:2}", p as usize - base),
            None => println!("reservar({tam:2}, alinea {align}) → sin memoria"),
        }
    }
    b.reiniciar();
    println!("tras reiniciar: {:?}", b.reservar(16, 8).map(|p| p as usize - base));
}
