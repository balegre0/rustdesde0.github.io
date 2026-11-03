use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

// 1. Un asignador que delega en el del sistema y cuenta
struct Contador;
static RESERVAS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Contador {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        RESERVAS.fetch_add(1, Relaxed);
        BYTES.fetch_add(layout.size(), Relaxed);
        // SAFETY: mismo contrato que el nuestro; lo pasamos tal cual
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: ptr vino de System.alloc con este mismo layout
        unsafe { System.dealloc(ptr, layout) }
    }
}

// 2. Instalarlo para TODO el programa (también lo usa std)
#[global_allocator]
static GLOBAL: Contador = Contador;

fn medir(nombre: &str, f: impl FnOnce()) {
    let (r0, b0) = (RESERVAS.load(Relaxed), BYTES.load(Relaxed));
    f();
    println!("{nombre:<28} {} reserva(s), {} bytes", RESERVAS.load(Relaxed) - r0, BYTES.load(Relaxed) - b0);
}

fn main() {
    medir("Vec::with_capacity(10) de u32", || { let v: Vec<u32> = Vec::with_capacity(10); drop(v); });
    medir("String::from(\"ñandú\")", || { let s = String::from("ñandú"); drop(s); });
    medir("push x 5 sin capacidad", || { let mut v = Vec::new(); for i in 0..5u64 { v.push(i); } drop(v); });
    medir("Box::new(()) (tamaño 0)", || { let b = Box::new(()); drop(b); });
    medir("let x = [0u8; 1000] (pila)", || { let x = [0u8; 1000]; std::hint::black_box(&x); });
}
