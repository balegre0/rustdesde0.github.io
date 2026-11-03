fn main() {
    let area: i32 = {
        let ancho = 4;
        let alto = 3;
        ancho * alto // sin `;`: este es el valor del bloque
    };
    println!("área = {area}");
}
