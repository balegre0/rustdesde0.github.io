use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};

// Estas funciones no hacen nada al ejecutarse:
// solo COMPILAN si T cumple el trait. La prueba la hace el compilador.
fn es_send<T: Send>() {}
fn es_sync<T: Sync>() {}

#[allow(dead_code)]
struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    // 1. Tipos «normales»: Send y Sync automáticamente.
    es_send::<i32>();
    es_sync::<i32>();
    es_send::<String>();
    es_sync::<Vec<u8>>();
    es_send::<Punto>(); // nadie escribió `impl Send for Punto`
    es_sync::<Punto>();

    // 2. Cell y RefCell: se pueden MOVER a otro hilo (Send)...
    es_send::<Cell<i32>>();
    es_send::<RefCell<i32>>();
    // ...pero no COMPARTIR (no son Sync): es_sync::<Cell<i32>>() no compilaría.

    // 3. Arc<Mutex<T>>: se puede mover y compartir.
    es_send::<Arc<Mutex<Vec<i32>>>>();
    es_sync::<Arc<Mutex<Vec<i32>>>>();

    // 4. Mutex convierte algo que no es Sync en algo que sí lo es.
    es_sync::<Mutex<Cell<i32>>>();

    println!("Todas las comprobaciones se hicieron al COMPILAR.");
}
