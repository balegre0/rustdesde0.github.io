#[derive(Debug, PartialEq)]
struct Opciones {
    verboso: bool,
    max: Option<u32>,
    origen: String,
    destino: String,
}

fn parsear(args: &[&str]) -> Result<Opciones, String> {
    todo!()
}
