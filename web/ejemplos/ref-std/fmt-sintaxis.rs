fn main() {
    let nombre = "Ana";
    let pi = 3.14159265;
    // 1. posiciones y nombres
    println!("{} y {}", "uno", "dos");
    println!("{1} y {0}", "uno", "dos");
    println!("{nombre} tiene {edad} años", edad = 31);
    // 2. ancho, alineación y relleno
    println!("[{:>8}]", nombre);
    println!("[{:<8}]", nombre);
    println!("[{:^8}]", nombre);
    println!("[{:*^8}]", nombre);
    println!("[{:05}]", 42);
    // 3. precisión
    println!("{:.2} {:8.3} {:+}", pi, pi, 7);
    // 4. bases
    println!("{:b} {:o} {:x} {:X} {:#x} {:#010b}", 10, 10, 255, 255, 255, 5);
    // 5. Debug y Debug bonito
    let v = vec![(1, "a"), (2, "b")];
    println!("{:?}", v);
    println!("{:#?}", v[0]);
    // 6. ancho y precisión desde variables
    let ancho = 6;
    println!("[{:>ancho$}] [{:.*}]", 1, 3, pi);
    // 7. llaves literales
    println!("{{}} {}", "y valor");
    println!("{:e}", 1234.5);
}
