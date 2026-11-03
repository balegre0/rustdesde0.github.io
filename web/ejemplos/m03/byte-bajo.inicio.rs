fn main() {
    let color: u32 = 0x12AB34CD;
    // Usando `as u8` (que se queda con el byte de abajo)
    // y el desplazamiento `>>`, imprime los cuatro bytes:
    // cd 34 ab 12
    let b0 = color as u8;
    println!("{b0:x}");
}
