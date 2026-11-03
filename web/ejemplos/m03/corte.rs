fn main() {
    let palabra = "año";
    // 1. Cortar por rangos de BYTES: "a" son los bytes 0..1
    println!("{}", &palabra[0..1]);
    // 2. "añ" son los bytes 0..3 (la ñ ocupa 2)
    println!("{}", &palabra[0..3]);
    // 3. 0..2 corta la ñ por la mitad
    println!("{}", &palabra[0..2]);
}
