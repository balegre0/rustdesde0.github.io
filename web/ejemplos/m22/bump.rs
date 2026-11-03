use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

// Un asignador «bump» (de avance): reparte trozos de un array fijo, siempre hacia delante.
const TAM: usize = 64 * 1024;

struct Bump {
    memoria: UnsafeCell<[u8; TAM]>,
    siguiente: AtomicUsize, // primer byte libre
    vivos: AtomicUsize,     // reservas aún no liberadas
}

// SAFETY: solo tocamos `memoria` a través de índices repartidos de forma atómica.
unsafe impl Sync for Bump {}

unsafe impl GlobalAlloc for Bump {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let base = self.memoria.get() as usize;
        let mut actual = self.siguiente.load(Ordering::Relaxed);
        loop {
            // 1. Redondear la DIRECCIÓN al alineamiento pedido
            let inicio = (base + actual).next_multiple_of(layout.align()) - base;
            let fin = match inicio.checked_add(layout.size()) {
                Some(f) if f <= TAM => f,
                _ => return std::ptr::null_mut(), // 2. Sin memoria: devolver nulo
            };
            // 3. Avanzar el puntero de forma atómica
            match self.siguiente.compare_exchange_weak(actual, fin, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => {
                    self.vivos.fetch_add(1, Ordering::Relaxed);
                    return (base + inicio) as *mut u8;
                }
                Err(otro) => actual = otro,
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // 4. Liberar no devuelve nada... salvo que ya no quede ninguna reserva viva
        if self.vivos.fetch_sub(1, Ordering::Relaxed) == 1 {
            self.siguiente.store(0, Ordering::Relaxed);
        }
    }
}

#[global_allocator]
static ASIGNADOR: Bump = Bump {
    memoria: UnsafeCell::new([0; TAM]),
    siguiente: AtomicUsize::new(0),
    vivos: AtomicUsize::new(0),
};

fn usados() -> usize {
    ASIGNADOR.siguiente.load(Ordering::Relaxed)
}

fn main() {
    let antes = usados();
    let a = Box::new(1u8);
    let tras_a = usados();
    let b = Box::new(7u64); // pide alineamiento 8: puede dejar un hueco
    let tras_b = usados();
    println!("Box<u8>  ocupó {} byte(s)", tras_a - antes);
    println!("Box<u64> avanzó {} byte(s) (hueco de alineamiento incluido)", tras_b - tras_a);
    println!("alineado a 8: {}", (&*b as *const u64 as usize) % 8 == 0);
    drop(a);
    drop(b);

    let v: Vec<u32> = (0..100).collect();
    println!("suma = {}, el asignador lleva {} bytes repartidos (también los que pidió std para stdout)", v.iter().sum::<u32>(), usados());
    let s = format!("{} elementos", v.len());
    println!("{s}");
}
