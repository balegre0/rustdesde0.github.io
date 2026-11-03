// Tabla calculada al compilar con una const fn que usa bucles y &mut.
const fn tabla_cuadrados<const N: usize>() -> [u32; N] {
    let mut t = [0u32; N];
    let mut i = 0;
    while i < N {
        t[i] = (i * i) as u32;
        i += 1;
    }
    t
}

const CUADRADOS: [u32; 6] = tabla_cuadrados();

// static: una única dirección; const: se copia en cada uso.
static SALUDO: &str = "hola";

const fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    println!("{CUADRADOS:?}");
    // Comprobación al compilar dentro de una función.
    const { assert!(CUADRADOS[3] == 9) };
    // La misma const fn también se puede llamar al ejecutar.
    let n = std::hint::black_box(10);
    println!("{} {}", factorial(20), factorial(n));
    // Promoción: `&5` vive para siempre (&'static i32).
    let r: &'static i32 = &5;
    println!("{r} {SALUDO}");
    // Longitud de array: contexto constante.
    let a = [0u8; factorial(4) as usize];
    println!("{}", a.len());
    // Un array de valores no Copy con `const {}`.
    let v: [Option<String>; 3] = [const { None }; 3];
    println!("{v:?}");
}
