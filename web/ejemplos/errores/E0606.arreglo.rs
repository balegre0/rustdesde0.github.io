fn main() {
    let x = &7u8;
    let y = *x as u32; // primero se lee el valor y luego se convierte
    println!("{y}");
}
