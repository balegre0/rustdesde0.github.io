trait Describir {
    fn describir(&self) -> String;
}

impl Describir for i32 {
    fn describir(&self) -> String {
        String::from("un entero")
    }
}

impl Describir for &str {
    fn describir(&self) -> String {
        format!("el texto {:?}", self)
    }
}

fn main() {
    println!("{}", 5.describir());
    println!("{}", "hola".describir());
}
