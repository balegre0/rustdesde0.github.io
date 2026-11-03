fn hacer_contador(inicio: i32, paso: i32) -> impl FnMut() -> i32 {
    let mut actual = inicio;
    move || {
        let valor = actual;
        actual += paso;
        valor
    }
}
