
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
    let texto = String::from("uno\ndos\ntres\ndoce");
    let r = lineas_con(&texto, "do");
    comprobar("patrón \"do\"", r == ["dos", "doce"], format!("obtuve {r:?}"));
    let r = lineas_con(&texto, "z");
    comprobar("sin coincidencias", r.is_empty(), format!("obtuve {r:?}"));
    let resultado;
    {
        // El patrón muere antes de usar el resultado: solo compila si patron no está atado a 'a.
        let patron = String::from("es");
        resultado = lineas_con(&texto, &patron);
    }
    comprobar("el patrón puede morir antes", resultado == ["tres"], format!("obtuve {resultado:?}"));
    let r = lineas_con("año\nmaño\nojo", "ño");
    comprobar("con ñ", r == ["año", "maño"], format!("obtuve {r:?}"));
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
