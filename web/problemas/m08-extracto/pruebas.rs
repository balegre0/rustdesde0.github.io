
fn main() {
    let mut fallos = 0;
    let mut comprobar = |nombre: &str, ok: bool, detalle: String| {
        if ok {
            println!("OK: {nombre}");
        } else {
            println!("FALLO: {nombre}: {detalle}");
            fallos += 1;
        }
    };
    let novela = String::from("Era una vez. Fin.");
    let frase;
    {
        let e = Extracto { texto: &novela };
        let n = e.palabras();
        comprobar("palabras = 4", n == 4, format!("obtuve {n}"));
        frase = e.primera_frase();
    } // el Extracto muere aquí; la frase sigue valiendo porque mira a `novela`
    comprobar("primera_frase", frase == "Era una vez", format!("obtuve {frase:?}"));
    let e = Extracto { texto: "sin punto final" };
    comprobar("sin punto", e.primera_frase() == "sin punto final", format!("obtuve {:?}", e.primera_frase()));
    let e = Extracto { texto: "" };
    comprobar("texto vacío: 0 palabras", e.palabras() == 0, format!("obtuve {}", e.palabras()));
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
