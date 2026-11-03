// `safe` solo tiene sentido dentro de un bloque `unsafe extern`.
safe fn f() {}

fn main() {
    f();
}
