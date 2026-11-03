fn es_bisiesto(año: u32) -> bool {
    año % 4 == 0 // incompleto: olvida la regla de los siglos
}

fn main() {
    let año = 1900;
    assert!(
        !es_bisiesto(año),
        "{año} no es bisiesto, pero es_bisiesto dice que sí"
    );
    println!("todo bien");
}
