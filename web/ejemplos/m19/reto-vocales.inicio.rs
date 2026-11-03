// Cuenta las vocales (con y sin tilde) de la frase usando matches!.
fn es_vocal(c: char) -> bool {
    false // cámbialo: usa matches!
}

fn main() {
    let frase = "El pingüino comió atún";
    let n = frase.chars().filter(|&c| es_vocal(c)).count();
    println!("{} vocales", n);
}
