mod geometria {
    pub fn donde() -> String {
        format!("{} · {}:{}:{}", module_path!(), file!(), line!(), column!())
    }
}

fn main() {
    println!("{}", geometria::donde());
    println!("estoy en la línea {}", line!());
}
