fn main() {
    // Into necesita saber el tipo de DESTINO: lo da la anotación.
    let x: u32 = 5u8.into();
    // O, más claro aún, From con el destino delante.
    let y = u64::from(5u8);
    println!("{x} {y}");
}
