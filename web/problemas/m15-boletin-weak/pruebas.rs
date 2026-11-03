
fn main() {
    let mut fallos = 0;
    let mut comprueba = |nombre: &str, ok: bool, detalle: String| {
        if ok {
            println!("OK: {nombre} ({detalle})");
        } else {
            println!("FALLO: {nombre} ({detalle})");
            fallos += 1;
        }
    };
    let mut b = Boletin { suscriptores: Vec::new() };
    let ana: Buzon = Rc::new(RefCell::new(Vec::new()));
    let luis: Buzon = Rc::new(RefCell::new(Vec::new()));
    b.suscribir(&ana);
    b.suscribir(&luis);
    comprueba("el boletín no es dueño", Rc::strong_count(&ana) == 1, format!("strong_count(ana) = {}", Rc::strong_count(&ana)));
    comprueba("dos activos", b.activos() == 2, format!("activos = {}", b.activos()));
    let n = b.publicar("hola");
    comprueba("publicar(hola) llega a 2", n == 2, format!("devolvió {n}"));
    let buzon_luis = luis.borrow().clone();
    comprueba("luis recibió hola", buzon_luis == ["hola"], format!("{buzon_luis:?}"));
    drop(luis);
    comprueba("tras irse luis, activos = 1", b.activos() == 1, format!("activos = {}", b.activos()));
    let n = b.publicar("adiós");
    comprueba("publicar(adiós) llega a 1", n == 1, format!("devolvió {n}"));
    comprueba("el Weak de luis se eliminó", b.suscriptores.len() == 1, format!("suscriptores.len() = {}", b.suscriptores.len()));
    let buzon_ana = ana.borrow().clone();
    comprueba("ana tiene los dos mensajes", buzon_ana == ["hola", "adiós"], format!("{buzon_ana:?}"));
    drop(ana);
    let n = b.publicar("¿hay alguien?");
    comprueba("sin nadie, llega a 0", n == 0 && b.suscriptores.is_empty(), format!("devolvió {n}, quedan {}", b.suscriptores.len()));
    if fallos == 0 {
        println!("TODOS LOS CASOS SUPERADOS");
    }
}
