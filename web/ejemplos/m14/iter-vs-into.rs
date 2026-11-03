fn main() {
    let mut palabras = vec![String::from("sol"), String::from("mar")];

    // 1. iter(): préstamos compartidos (&String). `palabras` sigue siendo tuyo.
    for p in palabras.iter() {
        println!("leo {p}");
    }

    // 2. iter_mut(): préstamos exclusivos (&mut String). Puedes cambiar cada elemento.
    for p in palabras.iter_mut() {
        p.push('!');
    }
    println!("{palabras:?}");

    // 3. into_iter(): te llevas cada String (se MUEVE). Después, `palabras` ya no existe.
    for p in palabras.into_iter() {
        let mayus: String = p.to_uppercase();
        println!("me quedo {mayus}");
    }
}
