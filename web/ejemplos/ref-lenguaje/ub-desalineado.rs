fn main() {
    let bytes = [0u8; 8];
    // Un puntero a u32 que empieza en una dirección impar: desalineado.
    let p = unsafe { bytes.as_ptr().add(1) } as *const u32;
    // Leer a través de un puntero desalineado es UB.
    let x = unsafe { *p };
    println!("{x}");
}
