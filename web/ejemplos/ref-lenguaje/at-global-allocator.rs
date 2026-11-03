use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// Un asignador que cuenta cuántas reservas se hacen y delega en el del sistema.
struct Contador;

static RESERVAS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Contador {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        RESERVAS.fetch_add(1, Ordering::Relaxed);
        // SAFETY: se reenvía el mismo layout que nos dieron.
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: ptr salió de System.alloc con este layout.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ASIGNADOR: Contador = Contador;

fn main() {
    let antes = RESERVAS.load(Ordering::Relaxed);
    let v: Vec<u64> = (0..100).collect();
    let s = String::from("hola");
    let despues = RESERVAS.load(Ordering::Relaxed);
    println!("{} elementos, «{s}»: {} reservas nuevas", v.len(), despues - antes);
}
