const CONST: [u32; 4] = al_reves![10, 20, 30, 40];

fn main() {
    let mut ok = true;
    let mut caso = |nombre: &str, bien: bool| {
        if bien { println!("OK: {}", nombre); } else { println!("FALLO: {}", nombre); ok = false; }
    };
    caso("tres números", al_reves![1, 2, 3] == [3, 2, 1]);
    caso("un elemento", al_reves!["a"] == ["a"]);
    caso("expresiones y coma final", al_reves![1 + 1, 2 * 5, 7,] == [7, 10, 2]);
    caso("en una constante", CONST == [40, 30, 20, 10]);
    caso("caracteres", al_reves!['ñ', 'u'] == ['u', 'ñ']);
    if ok { println!("TODOS LOS CASOS SUPERADOS"); }
}
