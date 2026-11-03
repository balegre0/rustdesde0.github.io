struct Caja(&'static str);

impl Drop for Caja {
    fn drop(&mut self) {
        println!("libero {}", self.0);
    }
}

fn main() {
    let _a = Caja("a");
    {
        let _b = Caja("b");
        println!("fin del bloque");
    }
    let _c = Caja("c");
    println!("fin de main");
}
