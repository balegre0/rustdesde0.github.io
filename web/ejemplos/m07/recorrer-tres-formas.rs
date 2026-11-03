fn main() {
    let mut nombres = vec![String::from("ana"), String::from("bea")];

    // 1. &v: préstamo compartido, cada x es &String
    for x in &nombres {
        print!("{x} ");
    }
    println!("← leídos");

    // 2. &mut v: préstamo exclusivo, cada x es &mut String
    for x in &mut nombres {
        x.make_ascii_uppercase();
    }
    println!("{nombres:?} ← modificados");

    // 3. v: se MUEVE el vector al bucle, cada x es String
    for x in nombres {
        print!("{} ", x.len());
    }
    println!("← consumidos");
    // aquí `nombres` ya no se puede usar
}
