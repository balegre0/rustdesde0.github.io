// Cada dato vive en algún lugar de la memoria. {:p} muestra su dirección.
fn main() {
    // 1. Tres datos de un byte cada uno
    let x: u8 = 10;
    let y: u8 = 20;
    let z: u8 = 30;

    // 2. Sus direcciones (los números cambian según la máquina; no los memorices)
    println!("x vale {x} y vive en {:p}", &x);
    println!("y vale {y} y vive en {:p}", &y);
    println!("z vale {z} y vive en {:p}", &z);

    // 3. Una dirección es un número: mide lo mismo que un usize
    println!("una dirección mide {} bytes", std::mem::size_of::<&u8>());
    println!("un usize mide {} bytes", std::mem::size_of::<usize>());
}
