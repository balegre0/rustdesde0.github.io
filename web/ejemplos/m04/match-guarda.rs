fn main() {
    for n in [-4, 0, 7, 12] {
        // Un brazo puede llevar una condición extra: `if` después del patrón
        let descripcion = match n {
            0 => String::from("cero"),
            x if x < 0 => format!("{x} es negativo"),
            x if x % 2 == 0 => format!("{x} es par"),
            x => format!("{x} es impar"),
        };
        println!("{descripcion}");
    }
}
