use std::collections::BTreeMap;

// 1. Cada repetición captura DOS fragmentos: la clave y el valor
macro_rules! mapa {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = BTreeMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

fn main() {
    let edades = mapa! {
        "Ana" => 31,
        "Íker" => 27,
        "Bea" => 45,
    };
    println!("{:?}", edades);
}
