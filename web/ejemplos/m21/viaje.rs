// Un programa pequeño que recorre TODO el pipeline del compilador.
fn main() {
    // 1. Un array de tres números (el lexer verá: `let`, `v`, `=`, `[`, `10`, ...)
    let v = [10, 20, 30];
    // 2. Un `for`: tras el *lowering* a HIR ya no existe; se convierte en loop + match
    for x in v {
        // 3. `println!` es una macro: se expande antes de comprobar tipos
        println!("{x}");
    }
}
