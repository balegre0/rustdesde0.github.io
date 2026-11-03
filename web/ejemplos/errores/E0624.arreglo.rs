mod motor {
    pub struct Motor;
    impl Motor {
        pub fn calentar(&self) {
            println!("calentando");
        }
    }
}

fn main() {
    let m = motor::Motor;
    m.calentar();
}
