// Jefe: un vector propio, unsafe por dentro y sound por fuera.
use std::alloc::{self, Layout};
use std::ptr::NonNull;

/// Invariantes (de seguridad, protegidas por campos privados):
/// - `ptr` apunta a una asignación de `cap` elementos (o es dangling si cap == 0);
/// - las posiciones `0..len` están inicializadas; `len <= cap`.
pub struct MiVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> MiVec<T> {
    pub fn new() -> Self {
        assert!(size_of::<T>() != 0, "este MiVec no admite tipos de tamaño cero");
        MiVec { ptr: NonNull::dangling(), cap: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn crecer(&mut self) {
        let nueva_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let nuevo = Layout::array::<T>(nueva_cap).expect("capacidad demasiado grande");
        let p = if self.cap == 0 {
            // SAFETY: el layout no tiene tamaño cero (T no es ZST y nueva_cap > 0).
            unsafe { alloc::alloc(nuevo) }
        } else {
            let viejo = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: ptr se asignó con `viejo`; el tamaño nuevo no es cero.
            unsafe { alloc::realloc(self.ptr.as_ptr().cast(), viejo, nuevo.size()) }
        };
        self.ptr = NonNull::new(p.cast()).unwrap_or_else(|| alloc::handle_alloc_error(nuevo));
        self.cap = nueva_cap;
    }

    pub fn push(&mut self, valor: T) {
        if self.len == self.cap {
            self.crecer();
        }
        // SAFETY: len < cap, así que la casilla está dentro de la asignación;
        // write no lee ni destruye lo que había (memoria sin inicializar).
        unsafe { self.ptr.as_ptr().add(self.len).write(valor) };
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: la casilla `len` estaba inicializada; al bajar len, la
        // «movemos» fuera y nadie la volverá a leer ni a destruir.
        Some(unsafe { self.ptr.as_ptr().add(self.len).read() })
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.len {
            // SAFETY: i < len: casilla inicializada; el préstamo queda atado a &self.
            Some(unsafe { &*self.ptr.as_ptr().add(i) })
        } else {
            None
        }
    }
}

impl<T> Drop for MiVec<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {} // destruye cada elemento vivo
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: ptr se asignó con exactamente este layout.
            unsafe { alloc::dealloc(self.ptr.as_ptr().cast(), layout) };
        }
    }
}

fn main() {
    let mut v = MiVec::new();
    for palabra in ["uno", "dos", "tres", "cuatro", "cinco"] {
        v.push(String::from(palabra)); // 1. crece dos veces: 0 → 4 → 8
    }
    println!("len = {}", v.len());
    println!("get(2) = {:?}, get(9) = {:?}", v.get(2), v.get(9));
    println!("pop = {:?}", v.pop());
    println!("len = {}", v.len());
} // 2. Drop libera los 4 String restantes y la memoria
