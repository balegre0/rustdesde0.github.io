fn main() {
    // 1. Precisión: cuántos decimales.
    println!("{:.2}", 3.14159);
    println!("{:.0}", 2.5);
    println!("{:8.3}|", 3.14159);
    // 2. Bases: binario, octal, hexadecimal.
    println!("{:b} {:o} {:x} {:X}", 202, 202, 202, 202);
    // 3. # añade el prefijo; 0 y un ancho rellenan con ceros.
    println!("{:#b} {:#x}", 202, 202);
    println!("{:08b}", 5);
    // 4. + muestra siempre el signo.
    println!("{:+} {:+}", 7, -7);
}
