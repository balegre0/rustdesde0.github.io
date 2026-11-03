// 1. Cada parámetro lleva nombre y tipo, separados por `:`
fn presentar(nombre: &str, edad: u32) {
    println!("{nombre} tiene {edad} años");
}

fn dibujar_linea(largo: usize, simbolo: char) {
    for _ in 0..largo {
        print!("{simbolo}");
    }
    println!();
}

fn main() {
    // 2. Los argumentos se pasan en el mismo orden que los parámetros
    presentar("Lucía", 31);
    presentar("Íñigo", 8);
    dibujar_linea(10, '=');
    dibujar_linea(3, 'ñ');
}
