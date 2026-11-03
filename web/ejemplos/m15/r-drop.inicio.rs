struct Vela(u32);

// 1. Implementa Drop para que imprima «apagada la vela N».
impl Drop for Vela {
    fn drop(&mut self) {
        todo!()
    }
}

fn main() {
    let _v1 = Vela(1);
    let v2 = Vela(2);
    let _v3 = Vela(3);
    // 2. Haz que la vela 2 se apague antes de «soplando...».
    let _ = &v2;
    println!("soplando...");
}
