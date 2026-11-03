use std::sync::Mutex;

struct Cuenta {
    id: u32,
    saldo: Mutex<u64>,
}

fn transferir(a: &Cuenta, b: &Cuenta, cantidad: u64) -> Result<(), String> {
    if a.id == b.id {
        return Err(String::from("misma cuenta"));
    }
    // Orden global de adquisición: primero el id menor.
    let (primera, segunda) = if a.id < b.id { (a, b) } else { (b, a) };
    let mut g1 = primera.saldo.lock().unwrap();
    let mut g2 = segunda.saldo.lock().unwrap();
    let (origen, destino) = if a.id < b.id { (&mut *g1, &mut *g2) } else { (&mut *g2, &mut *g1) };
    if *origen < cantidad {
        return Err(String::from("saldo insuficiente"));
    }
    *origen -= cantidad;
    *destino += cantidad;
    Ok(())
}
