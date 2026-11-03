
fn main() {
    let casos = [
        ("09:30", Ok(Hora { h: 9, m: 30 })),
        ("23:59", Ok(Hora { h: 23, m: 59 })),
        ("00:00", Ok(Hora { h: 0, m: 0 })),
        ("24:00", Err(ErrorHora::HoraInvalida(24))),
        ("12:60", Err(ErrorHora::MinutoInvalido(60))),
        ("1230", Err(ErrorHora::Formato)),
        ("12:3:4", Err(ErrorHora::Formato)),
        ("aa:bb", Err(ErrorHora::Formato)),
        ("300:00", Err(ErrorHora::Formato)),
        ("", Err(ErrorHora::Formato)),
    ];
    let mut fallos = 0;
    for (texto, esperado) in casos {
        let obtenido = parsear_hora(texto);
        if obtenido == esperado {
            println!("OK: {:?} -> {:?}", texto, obtenido);
        } else {
            println!("FALLO: {:?}: esperaba {:?}, obtuve {:?}", texto, esperado, obtenido);
            fallos += 1;
        }
    }
    let mensajes = [
        (ErrorHora::Formato, "usa el formato HH:MM"),
        (ErrorHora::HoraInvalida(25), "la hora 25 no existe"),
        (ErrorHora::MinutoInvalido(61), "el minuto 61 no existe"),
    ];
    for (e, esperado) in mensajes {
        let obtenido = e.to_string();
        if obtenido == esperado {
            println!("OK: Display de {:?} = {:?}", e, obtenido);
        } else {
            println!("FALLO: Display de {:?}: esperaba {:?}, obtuve {:?}", e, esperado, obtenido);
            fallos += 1;
        }
    }
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
