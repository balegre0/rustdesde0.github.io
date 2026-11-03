trait Describir {
    fn describir(&self) -> String;
}

// Una impl general («blanket»): para TODO tipo que sea Debug.
impl<T: std::fmt::Debug> Describir for T {
    fn describir(&self) -> String {
        format!("{self:?}")
    }
}

// Y otra para i32… que también es Debug: ¿cuál usar?
impl Describir for i32 {
    fn describir(&self) -> String {
        String::from("un entero")
    }
}

fn main() {
    println!("{}", 5.describir());
}
