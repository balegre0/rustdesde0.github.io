mod cocina {
    pub fn preparar() -> String {
        horno::hornear("pan")
    }

    mod horno {
        pub fn hornear(que: &str) -> String {
            format!("{que} horneado")
        }
    }
}

fn main() {
    println!("{}", cocina::preparar());
    println!("{}", cocina::horno::hornear("bizcocho"));
}
