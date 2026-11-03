fn main() {
    let nombre = "Ana";
    let puntos = 9.5;
    println!(); // solo un salto de línea
    println!("hola");
    println!("{nombre} tiene {puntos} puntos");
    println!("{} tiene {:.0} puntos", nombre, puntos);
    print!("sin ");
    print!("salto");
    println!();
    eprintln!("esto va a stderr");
    eprint!("y esto también\n");
}
