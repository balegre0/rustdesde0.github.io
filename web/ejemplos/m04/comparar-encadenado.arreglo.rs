fn main() {
    let edad = 15;
    if 12 < edad && edad < 18 {
        println!("adolescente");
    }
    // También con un rango:
    if (13..18).contains(&edad) {
        println!("adolescente (con un rango)");
    }
}
