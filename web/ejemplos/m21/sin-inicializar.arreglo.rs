fn main() {
    let llueve = true;
    // Todas las ramas dan valor: el análisis de flujo de MIR queda contento.
    let x: i32 = if llueve { 1 } else { 0 };
    println!("{x}");
}
