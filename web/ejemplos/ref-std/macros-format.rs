fn main() {
    let x = 3;
    let s: String = format!("{x} al cuadrado es {}", x * x);
    println!("{s}");
    let tabla: Vec<String> = (1..=3).map(|i| format!("{i:>2} | {:>3}", i * 10)).collect();
    println!("{}", tabla.join("\n"));
    // format_args! no reserva memoria: se usa dentro de otras macros
    println!("{}", format_args!("{}-{}", 'a', 'b'));
    let n = 255;
    println!("{}", format!("{n:#x} {n:#b}"));
}
