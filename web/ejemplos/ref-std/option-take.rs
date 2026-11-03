fn main() {
    let mut turno: Option<String> = Some(String::from("Ana"));
    // take() saca el valor y deja None en su lugar
    let actual = turno.take();
    println!("{actual:?} {turno:?}");
    // replace() pone un valor nuevo y devuelve el viejo
    let viejo = turno.replace(String::from("Luis"));
    println!("{viejo:?} {turno:?}");
    // as_ref() permite mirar sin mover
    let largo = turno.as_ref().map(|s| s.len());
    println!("{largo:?} {turno:?}");
    // ok_or convierte en Result
    let r: Result<String, &str> = None.ok_or("vacío");
    println!("{r:?}");
}
