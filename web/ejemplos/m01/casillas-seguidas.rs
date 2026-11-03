// Un array guarda sus elementos en casillas seguidas de memoria.
fn main() {
    let numeros: [u32; 3] = [100, 200, 300];

    // 1. La dirección de cada elemento
    let d0 = &numeros[0] as *const u32 as usize;
    let d1 = &numeros[1] as *const u32 as usize;
    let d2 = &numeros[2] as *const u32 as usize;

    // 2. Lo que importa no es el número, sino la distancia entre ellos
    println!("de [0] a [1] hay {} bytes", d1 - d0);
    println!("de [1] a [2] hay {} bytes", d2 - d1);
    println!("un u32 mide {} bytes", std::mem::size_of::<u32>());
}
