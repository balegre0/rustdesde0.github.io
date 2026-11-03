// El trait es NUESTRO, así que podemos implementarlo para tipos ajenos
// como i32 o str. Esto se llama «trait de extensión».
trait EsPar {
    fn es_par(&self) -> bool;
}

impl EsPar for i32 {
    fn es_par(&self) -> bool {
        self % 2 == 0
    }
}

trait Gritar {
    fn gritar(&self) -> String;
}

impl Gritar for str {
    fn gritar(&self) -> String {
        format!("¡{}!", self.to_uppercase())
    }
}

fn main() {
    println!("{} {}", 4.es_par(), 7.es_par());
    println!("{}", "ñandú".gritar());
}
