use std::ptr;

fn main() {
    let mut x = 10;
    // crear punteros crudos es seguro
    let p: *const i32 = &raw const x;
    let q: *mut i32 = &raw mut x;
    println!("{}", p.is_null());
    println!("{}", ptr::eq(p, q));
    // leer o escribir a través de ellos, no
    unsafe {
        // SEGURIDAD: q apunta a x, que está viva y no tiene otros préstamos activos.
        *q += 1;
        println!("{}", *p);
        let v = ptr::read(p);
        ptr::write(q, v * 2);
    }
    println!("{x}");
    let nulo: *const u8 = ptr::null();
    println!("{} {:?}", nulo.is_null(), unsafe { nulo.as_ref() });
    let mut caja = [1, 2, 3];
    // SEGURIDAD: los índices 0 y 2 están dentro del array.
    unsafe { ptr::swap(&raw mut caja[0], &raw mut caja[2]) };
    println!("{caja:?}");
}
