#[derive(Debug)]
#[allow(dead_code)]
enum Forma { Circulo { r: f64 }, Rect(f64, f64), Punto }

const LIMITE: i32 = 100;

fn main() {
    // Literal, rango (inclusivo y exclusivo), constante, alternativas.
    for n in [0, 5, 15, 100, -3] {
        let s = match n {
            0 => "cero",
            1..10 => "un dígito",
            10..=99 => "dos dígitos",
            LIMITE => "el límite",
            i32::MIN..0 | 101.. => "fuera",
        };
        println!("{n}: {s}");
    }
    // Struct, tupla struct, path; `..` para ignorar el resto.
    for f in [Forma::Circulo { r: 1.0 }, Forma::Rect(2.0, 3.0), Forma::Punto] {
        match f {
            Forma::Circulo { r } => println!("círculo r={r}"),
            Forma::Rect(ancho, ..) => println!("rect ancho={ancho}"),
            Forma::Punto => println!("punto"),
        }
    }
    // Slice: primero, resto con `@ ..`, último.
    let v = [1, 2, 3, 4];
    // Con un array de tamaño fijo este patrón no puede fallar: vale en un `let`.
    let [primero, medio @ .., ultimo] = v;
    println!("{primero} {medio:?} {ultimo}");
    // `@` enlaza el valor y además lo comprueba.
    let edad = 17;
    match edad {
        e @ 0..18 => println!("{e}: menor"),
        e => println!("{e}: adulto"),
    }
    // Referencia en el patrón y ergonomía de match.
    let pares = vec![(1, "uno"), (2, "dos")];
    for &(n, nombre) in &pares {
        print!("{n}={nombre} ");
    }
    for (n, nombre) in &pares {
        // aquí n: &i32, nombre: &&str (modo de enlace por referencia)
        print!("{}={} ", n + 0, nombre);
    }
    println!();
    // Patrón en parámetros de función.
    fn suma(&(a, b): &(i32, i32)) -> i32 { a + b }
    println!("{}", suma(&(3, 4)));
}
