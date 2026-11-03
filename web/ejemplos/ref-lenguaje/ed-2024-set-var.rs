fn main() {
    // Edición 2024: `set_var` es `unsafe`, porque cambiar el entorno
    // mientras otro hilo lo lee puede ser comportamiento indefinido.
    std::env::set_var("MODO", "prueba");
}
