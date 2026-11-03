// El puntero de un CString temporal queda colgando al acabar la sentencia.
use std::ffi::{c_char, CString};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    let p = CString::new("hola").unwrap().as_ptr(); // ¡MAL! el CString muere aquí
    let n = unsafe { strlen(p) }; // uso tras liberar
    println!("{n}");
}
