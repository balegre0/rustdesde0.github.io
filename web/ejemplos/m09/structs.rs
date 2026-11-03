// 1. Definir el molde: un struct con campos con nombre y tipo.
struct Libro {
    titulo: String,
    paginas: u32,
    leido: bool,
}

fn main() {
    // 2. Crear un valor: hay que dar TODOS los campos (en cualquier orden).
    let mut libro = Libro {
        paginas: 320,
        titulo: String::from("El año del dragón"),
        leido: false,
    };

    // 3. Leer campos con el punto.
    println!("«{}» tiene {} páginas", libro.titulo, libro.paginas);

    // 4. Cambiar un campo: el valor entero tiene que ser `mut`.
    libro.leido = true;
    println!("¿Leído? {}", libro.leido);

    // 5. Atajo de inicialización: si la variable se llama igual que el campo.
    let titulo = String::from("Ñandúes del sur");
    let paginas = 96;
    let otro = Libro { titulo, paginas, leido: false };
    println!("«{}»: {} páginas, leído = {}", otro.titulo, otro.paginas, otro.leido);
}
