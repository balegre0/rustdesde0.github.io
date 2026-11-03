// Edición 2024: `impl Trait` en el retorno captura todos los parámetros
// genéricos y tiempos de vida en ámbito (aquí, 'a). En 2021 daba E0700.
fn letras<'a>(s: &'a str) -> impl Iterator<Item = char> {
    s.chars()
}

// Con `use<>` se dice exactamente qué se captura: aquí, nada.
fn contador(n: u32) -> impl Iterator<Item = u32> + use<> {
    0..n
}

fn main() {
    println!("{}", letras("hola").collect::<String>());
    println!("{:?}", contador(3).collect::<Vec<_>>());
}
