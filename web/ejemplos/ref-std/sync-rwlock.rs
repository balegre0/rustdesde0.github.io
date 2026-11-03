use std::sync::RwLock;

fn main() {
    let config = RwLock::new(String::from("modo: normal"));
    {
        // varios lectores a la vez
        let r1 = config.read().unwrap();
        let r2 = config.read().unwrap();
        println!("{} | {}", *r1, *r2);
    }
    {
        // un solo escritor, sin lectores
        let mut w = config.write().unwrap();
        w.push_str(" (cambiado)");
    }
    println!("{}", config.read().unwrap());
    let r = config.read().unwrap();
    println!("{}", config.try_write().is_err());
    drop(r);
}
