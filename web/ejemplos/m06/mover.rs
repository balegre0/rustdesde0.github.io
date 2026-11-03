fn main() {
    // 1. `s1` es dueña de un texto en el heap
    let s1 = String::from("hola");
    let direccion_antes = s1.as_ptr();

    // 2. Movemos: la propiedad pasa de `s1` a `s2`
    let s2 = s1;
    let direccion_despues = s2.as_ptr();

    // 3. Los bytes "hola" siguen en el mismo sitio del heap
    println!("s2 = {s2}");
    println!("bytes antes:   {direccion_antes:p}");
    println!("bytes después: {direccion_despues:p}");
    println!("¿mismos bytes? {}", direccion_antes == direccion_despues);
}
