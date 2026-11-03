use std::sync::RwLock;

fn main() {
    let menu = RwLock::new(vec!["sopa", "tortilla"]);

    {
        // 1. Varios lectores a la vez: como varios préstamos compartidos.
        let l1 = menu.read().unwrap();
        let l2 = menu.read().unwrap();
        println!("lectores: {} y {}", l1.len(), l2.len());
        // 2. Con lectores vivos, un escritor no puede entrar.
        println!("¿puede escribir? {}", menu.try_write().is_ok());
    }

    {
        // 3. Un solo escritor, y nadie más: como un préstamo exclusivo.
        let mut e = menu.write().unwrap();
        e.push("flan");
        println!("¿puede leer otro? {}", menu.try_read().is_ok());
    }

    println!("menú: {:?}", menu.read().unwrap());
}
