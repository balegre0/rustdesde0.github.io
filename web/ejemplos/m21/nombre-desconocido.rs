fn main() {
    let total = 10;
    // La resolución de nombres busca `totl` en todos los ámbitos... y no existe.
    println!("{}", totl + 1);
}
