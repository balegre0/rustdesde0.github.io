
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

    let r = leer_punto("3, -4");
    comprobar("leer «3, -4»", r.as_ref().ok() == Some(&Punto { x: 3, y: -4 }), format!("{:?}", r));

    let r = leer_punto("3 4");
    let ok = matches!(r, Err(ErrorPunto::SinComa)) && r.as_ref().err().map(|e| e.to_string()) == Some(String::from("falta la coma"));
    comprobar("sin coma", ok, format!("{:?}", r));

    let r = leer_punto("tres,4");
    let ok = match &r {
        Err(e @ ErrorPunto::Coordenada { eje: 'x', .. }) => {
            e.to_string() == "coordenada x inválida"
                && e.source().map(|c| c.to_string()) == Some(String::from("invalid digit found in string"))
        }
        _ => false,
    };
    comprobar("x inválida con causa", ok, format!("{:?}", r));

    let r = leer_punto("3,99999999999");
    let ok = match &r {
        Err(e @ ErrorPunto::Coordenada { eje: 'y', .. }) => {
            e.to_string() == "coordenada y inválida"
                && e.source().map(|c| c.to_string()) == Some(String::from("number too large to fit in target type"))
        }
        _ => false,
    };
    comprobar("y demasiado grande con causa", ok, format!("{:?}", r));

    let r = leer_punto("1,2,3");
    comprobar("«1,2,3»: la y es «2,3»", matches!(r, Err(ErrorPunto::Coordenada { eje: 'y', .. })), format!("{:?}", r));

    comprobar("SinComa no tiene causa", ErrorPunto::SinComa.source().is_none(), String::from("source() debería ser None"));

    let d = distancia2("-100000,0", "100000,0");
    comprobar("distancia² con coordenadas grandes", d.as_ref().ok() == Some(&40000000000), format!("{:?}", d));

    let d = distancia2("0,0", "3,4");
    comprobar("distancia² (0,0)-(3,4) = 25", d.as_ref().ok() == Some(&25), format!("{:?}", d));


    let d = distancia2("0,0", "x,1");
    let ok = match &d {
        Err(e) => e.to_string() == "coordenada x inválida",
        Ok(_) => false,
    };
    comprobar("distancia² propaga el error", ok, format!("{:?}", d));

    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
