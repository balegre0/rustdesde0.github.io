// ¿Cuántos bytes mide una dirección de memoria en esta máquina?
fn main() {
    println!("{}", std::mem::size_of::<usize>());
}
