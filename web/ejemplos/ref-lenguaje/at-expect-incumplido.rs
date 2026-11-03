#![deny(unfulfilled_lint_expectations)]

fn main() {
    #[expect(unused_variables)] // ¡pero la variable SÍ se usa!
    let x = 1;
    println!("{x}");
}
