use std::marker::PhantomData;

// Estados como tipos: no ocupan nada, solo existen para el compilador.
struct Abierta;
struct Cerrada;

struct Puerta<Estado> {
    nombre: String,
    _estado: PhantomData<Estado>,
}

impl Puerta<Cerrada> {
    fn nueva(nombre: &str) -> Puerta<Cerrada> {
        Puerta { nombre: nombre.to_string(), _estado: PhantomData }
    }
    fn abrir(self) -> Puerta<Abierta> {
        println!("{} se abre", self.nombre);
        Puerta { nombre: self.nombre, _estado: PhantomData }
    }
}

impl Puerta<Abierta> {
    fn pasar(&self) {
        println!("pasas por {}", self.nombre);
    }
    fn cerrar(self) -> Puerta<Cerrada> {
        println!("{} se cierra", self.nombre);
        Puerta { nombre: self.nombre, _estado: PhantomData }
    }
}

fn main() {
    let p = Puerta::nueva("la entrada");
    let p = p.abrir();
    p.pasar();
    let p = p.cerrar();
    println!("bytes: {} (lo mismo que un String: {})", size_of_val(&p), size_of::<String>());
}
