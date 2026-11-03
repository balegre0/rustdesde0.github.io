use std::panic::Location;

#[track_caller]
fn donde() -> &'static Location<'static> {
    Location::caller() // con track_caller: la línea de QUIEN llama
}

fn sin_track() -> &'static Location<'static> {
    Location::caller() // sin track_caller: esta misma línea
}

fn main() {
    let a = donde();
    let b = sin_track();
    println!("donde() llamada en la línea {}", a.line());
    println!("sin_track() informa de la línea {}", b.line());
}
