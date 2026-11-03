macro_rules! mapa {
    ($($k:expr => $v:expr),* $(,)?) => {
        ::std::collections::HashMap::new() // cámbialo: inserta las parejas
    };
}
