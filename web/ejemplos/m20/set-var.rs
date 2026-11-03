// Edición 2024: cambiar variables de entorno es unsafe (otros hilos podrían leerlas).
fn main() {
    std::env::set_var("MODO", "pruebas");
    println!("{:?}", std::env::var("MODO"));
}
