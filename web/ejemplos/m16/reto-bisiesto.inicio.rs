// Las pruebas de main describen la regla completa del calendario.
// Arregla es_bisiesto para que pasen TODAS (ahora falla con 1900).
fn es_bisiesto(año: u32) -> bool {
    año % 4 == 0
}

fn main() {
    let casos = [(2024, true), (2023, false), (2000, true), (1900, false), (2100, false), (1600, true)];
    for (año, esperado) in casos {
        assert_eq!(es_bisiesto(año), esperado, "fallo con el año {año}");
        println!("{año}: ok");
    }
    println!("calendario correcto");
}
