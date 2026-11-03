// En la edición 2024, un bloque extern debe escribirse `unsafe extern`:
// el compilador no puede comprobar que las firmas declaradas sean ciertas.
extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {}
