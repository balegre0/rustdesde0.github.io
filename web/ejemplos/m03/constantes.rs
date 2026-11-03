// 1. const: un valor fijo que se decide al compilar.
//    Tipo obligatorio y nombre en MAYÚSCULAS.
const SEGUNDOS_POR_HORA: u32 = 60 * 60;
const MAX_VIDAS: u8 = 3;

// 2. static: un valor que vive en UN sitio fijo de memoria
//    durante todo el programa.
static SALUDO: &str = "¡Bienvenida!";

fn main() {
    println!("{SALUDO}");
    println!("Un día tiene {} segundos", 24 * SEGUNDOS_POR_HORA);
    println!("Empiezas con {MAX_VIDAS} vidas");

    // 3. Una const se puede usar para el tamaño de un array
    const N: usize = 4;
    let tablero = [0; N];
    println!("{:?}", tablero);
}
