struct T(&'static str);
impl Drop for T {
    fn drop(&mut self) { println!("  destruyo {}", self.0); }
}
impl T {
    fn valor(&self) -> u32 { 1 }
}

fn main() {
    println!("1. un temporal vive hasta el final de su sentencia");
    let n = T("temporal").valor();
    println!("  n = {n}");
    println!("2. extensión: `let r = &temporal;` alarga su vida hasta el final del bloque");
    {
        let _r = &T("extendido");
        println!("  (todavía vivo)");
    }
    println!("3. edición 2024: el temporal del escrutinio de `if let` muere antes del `else`");
    if let Some(_) = T("escrutinio").valor().checked_sub(5) {
        println!("  rama then");
    } else {
        println!("  rama else");
    }
    println!("fin");
}
