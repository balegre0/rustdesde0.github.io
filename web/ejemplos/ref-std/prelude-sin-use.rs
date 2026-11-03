// Nada de `use`: todo esto viene del preludio.
fn main() {
    let nombres: Vec<String> = vec![String::from("Ana"), "Luis".to_string()];
    let primero: Option<&String> = nombres.first();
    let numero: Result<i32, _> = "42".parse::<i32>();
    let copia = nombres.clone();
    drop(copia); // `drop` también es del preludio
    println!("{primero:?} {numero:?} {}", nombres.len());
}
