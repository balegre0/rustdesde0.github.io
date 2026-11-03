#[derive(Debug, Clone, Copy, PartialEq)]
enum Semaforo {
    Rojo,
    Ambar,
    Verde,
}

// Option<T> «es» T + 1 y Result<T, ()> también: las dos conversiones son inversas.
fn a_result<T>(o: Option<T>) -> Result<T, ()> {
    match o {
        Some(x) => Ok(x),
        None => Err(()),
    }
}

fn a_option<T>(r: Result<T, ()>) -> Option<T> {
    match r {
        Ok(x) => Some(x),
        Err(()) => None,
    }
}

fn main() {
    // |Semaforo| = 3, |bool| = 2 → |(Semaforo, bool)| = 3 × 2 = 6
    let mut n = 0;
    for s in [Semaforo::Rojo, Semaforo::Ambar, Semaforo::Verde] {
        for parpadea in [false, true] {
            let _ = (s, parpadea);
            n += 1;
        }
    }
    println!("|(Semaforo, bool)| = {n}");

    // |Option<Semaforo>| = 3 + 1 = 4
    let todos = [None, Some(Semaforo::Rojo), Some(Semaforo::Ambar), Some(Semaforo::Verde)];
    for o in todos {
        let ida_y_vuelta = a_option(a_result(o));
        assert_eq!(o, ida_y_vuelta);
    }
    println!("Option<T> y Result<T, ()> son isomorfos: {} valores, ida y vuelta sin perder nada", todos.len());
}
