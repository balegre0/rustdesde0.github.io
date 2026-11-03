use std::collections::HashMap;
use std::sync::{LazyLock, OnceLock};

// 1. LazyLock: se inicializa la PRIMERA vez que alguien lo usa.
static CAPITALES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    println!("(construyendo la tabla, solo una vez)");
    HashMap::from([("España", "Madrid"), ("Perú", "Lima")])
});

// 2. OnceLock: empieza vacío y se rellena una única vez.
static VERSION: OnceLock<String> = OnceLock::new();

fn main() {
    println!("antes de usar la tabla");
    println!("{}", CAPITALES["Perú"]);
    println!("{}", CAPITALES["España"]);

    VERSION.set(String::from("1.0")).unwrap();
    let otra_vez = VERSION.set(String::from("2.0"));
    println!("versión: {} (segundo set ok? {})", VERSION.get().unwrap(), otra_vez.is_ok());
}
