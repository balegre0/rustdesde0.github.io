struct Vela(u32);

impl Drop for Vela {
    fn drop(&mut self) {
        println!("apagada la vela {}", self.0);
    }
}

fn main() {
    let _v1 = Vela(1);
    let v2 = Vela(2);
    let _v3 = Vela(3);
    drop(v2); // la 2 se apaga la primera
    println!("soplando...");
}
