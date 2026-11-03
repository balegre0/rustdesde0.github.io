#[derive(Debug, Clone, Copy)]
struct Punto {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Etiqueta {
    texto: String,
}

fn main() {
    let a = Punto { x: 1, y: 2 };
    let b = a; // Copy: se copian los bytes, `a` sigue valiendo
    println!("{a:?} {b:?} {}", a.x + b.y);
    let e = Etiqueta { texto: String::from("hola") };
    let f = e.clone(); // Clone: copia explícita (aquí, del String)
    let g = e; // sin Copy, esto mueve
    println!("{f:?} {g:?} {}", g.texto.len());
}
