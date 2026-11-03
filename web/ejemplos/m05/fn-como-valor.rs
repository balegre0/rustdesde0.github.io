fn doble(x: i32) -> i32 {
    x * 2
}

fn cuadrado(x: i32) -> i32 {
    x * x
}

// Recibe una función como parámetro: su tipo es fn(i32) -> i32.
fn aplicar_dos_veces(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    println!("{}", aplicar_dos_veces(doble, 3));
    println!("{}", aplicar_dos_veces(cuadrado, 3));

    // Las funciones se pueden guardar en variables y en arrays.
    let operaciones: [(&str, fn(i32) -> i32); 2] = [("doble", doble), ("cuadrado", cuadrado)];
    for (nombre, op) in operaciones {
        println!("{nombre}(7) = {}", op(7));
    }
}
