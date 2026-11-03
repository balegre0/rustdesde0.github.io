fn main() {
    // 1. El temporal vive hasta el final de la sentencia... salvo que lo
    //    prestes directamente en un `let`: entonces se alarga hasta la `}`.
    let s = &String::from("hola");
    println!("{s}");

    // 2. Lo normal: dale un nombre al dueño y presta después.
    let dueno = String::from("adiós");
    let vista: &str = dueno.as_str();
    println!("{vista}");

    // 3. Usar el temporal dentro de la misma sentencia también vale.
    println!("{}", String::from("ñu").as_str().len());
}
