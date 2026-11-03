fn edad_valida(edad: i32) -> bool {
    if edad < 0 {
        return "no";
    }
    edad < 150
}

fn main() {
    println!("{}", edad_valida(30));
}
