use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn es_send<T: Send>() {}
fn es_sync<T: Sync>() {}
fn es_copy<T: Copy>() {}

fn main() {
    // si una línea no compilara, el tipo no tendría ese trait
    es_send::<i32>();
    es_sync::<i32>();
    es_send::<String>();
    es_send::<Arc<Mutex<Vec<u8>>>>();
    es_sync::<Arc<Mutex<Vec<u8>>>>();
    es_send::<RefCell<u8>>(); // RefCell es Send...
    // es_sync::<RefCell<u8>>();  ...pero no Sync
    // es_send::<Rc<u8>>();       Rc no es Send
    es_copy::<(i32, char, bool)>();
    es_copy::<&String>(); // las referencias compartidas son Copy
    // es_copy::<String>();       String no es Copy
    let _ = Rc::new(0);
    println!("todo comprobado al compilar");
}
