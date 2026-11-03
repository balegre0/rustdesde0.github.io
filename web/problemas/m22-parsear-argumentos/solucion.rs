#[derive(Debug, PartialEq)]
struct Opciones {
    verboso: bool,
    max: Option<u32>,
    origen: String,
    destino: String,
}

fn parsear(args: &[&str]) -> Result<Opciones, String> {
    let mut verboso = false;
    let mut max = None;
    let mut posicionales = Vec::new();
    let mut it = args.iter();
    while let Some(&a) = it.next() {
        match a {
            "-v" => verboso = true,
            "--max" => {
                let n = it.next().and_then(|n| n.parse::<u32>().ok());
                max = Some(n.ok_or_else(|| String::from("--max necesita un número"))?);
            }
            _ if a.starts_with('-') => return Err(format!("opción desconocida: {a}")),
            _ => posicionales.push(a.to_string()),
        }
    }
    let [origen, destino]: [String; 2] = posicionales
        .try_into()
        .map_err(|_| String::from("uso: copiar [-v] [--max N] ORIGEN DESTINO"))?;
    Ok(Opciones { verboso, max, origen, destino })
}
