trait Describir {
    fn describir(&self) -> String;
}

impl<T: std::fmt::Debug> Describir for T {
    fn describir(&self) -> String {
        format!("{:?}", self)
    }
}

impl Describir for i32 {
    fn describir(&self) -> String {
        String::from("un entero")
    }
}

fn main() {
    println!("{}", 5.describir());
}
