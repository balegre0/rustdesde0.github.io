fn main() {
    let a = 3;
    // 1. dbg! imprime en stderr el archivo, la línea, la expresión y su valor...
    // 2. ...y DEVUELVE el valor, así que se puede meter en medio de un cálculo
    let b = dbg!(a * 2) + 1;
    println!("b = {}", b);
}
