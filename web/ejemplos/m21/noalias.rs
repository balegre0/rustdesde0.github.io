// Dos funciones casi iguales. La diferencia: &mut promete «no hay nadie más mirando».
fn con_refs(a: &mut i32, b: &mut i32) -> i32 {
    *a = 1;
    *b = 2;
    *a // el compilador puede devolver 1 sin leer: a y b NUNCA son el mismo sitio
}

unsafe fn con_crudos(a: *mut i32, b: *mut i32) -> i32 {
    unsafe {
        *a = 1;
        *b = 2;
        *a // aquí hay que leer de verdad: a y b podrían apuntar al mismo i32
    }
}

fn main() {
    let mut x = 0;
    let mut y = 0;
    // 1. Con referencias exclusivas, x e y son por fuerza variables distintas
    println!("con_refs(&mut x, &mut y) = {}", con_refs(&mut x, &mut y));

    // 2. Con punteros crudos SÍ podemos pasar el mismo sitio dos veces
    let p = &raw mut x;
    println!("con_crudos(p, p)         = {}", unsafe { con_crudos(p, p) });
    println!("x = {x}, y = {y}");
}
