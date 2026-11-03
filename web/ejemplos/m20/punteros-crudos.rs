// Punteros crudos: crearlos es seguro; usarlos, no.
fn main() {
    let mut numeros = [1, 2, 3, 4];

    // 1. Dos formas de crear punteros crudos (ninguna necesita unsafe).
    let p: *const i32 = &numeros[0]; // coerción desde una referencia
    let q: *mut i32 = &raw mut numeros[3]; // directamente, sin referencia intermedia

    // 2. Un puntero puede ser nulo; una referencia, nunca.
    let nulo: *const i32 = std::ptr::null();
    println!("¿p es nulo? {}  ¿nulo es nulo? {}", p.is_null(), nulo.is_null());

    // 3. Aritmética: `add(i)` avanza i elementos (no i bytes).
    // SAFETY: p apunta a un array de 4 i32; p+2 sigue dentro.
    let tercero = unsafe { *p.add(2) };
    println!("tercero = {tercero}");

    // 4. Escribir a través de *mut.
    // SAFETY: q apunta a numeros[3] y nadie más lo está usando ahora.
    unsafe { *q = 40 };

    // 5. Distancia entre punteros, en elementos.
    // SAFETY: los dos punteros vienen del mismo array.
    let d = unsafe { (q as *const i32).offset_from(p) };
    println!("distancia = {d} elementos; array = {numeros:?}");
    println!("tamaño de *const i32: {} bytes", size_of::<*const i32>());
}
