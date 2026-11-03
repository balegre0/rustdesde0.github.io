use std::ptr::{self, NonNull};

fn main() {
    let mut x = 10;
    let p: *mut i32 = &mut x;
    // SAFETY: p apunta a x, que está viva y no tiene otros préstamos activos.
    unsafe {
        p.write(20);
        println!("leído: {}", p.read());
    }

    let nulo: *const i32 = ptr::null();
    println!("¿es nulo? {}", nulo.is_null());

    // NonNull: un puntero crudo que nunca es nulo (y por eso Option<NonNull<T>> no ocupa más).
    let nn = NonNull::new(p).expect("no es nulo");
    println!("Option<NonNull<i32>> mide lo mismo que *mut i32: {}",
        size_of::<Option<NonNull<i32>>>() == size_of::<*mut i32>());
    println!("NonNull::new(nulo) = {:?}", NonNull::new(nulo as *mut i32));
    let _ = nn;
}
