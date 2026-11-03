// 1. MAL: un struct con banderas permite estados absurdos.
#[allow(dead_code)]
struct ConexionMal {
    conectada: bool,
    autenticada: bool,     // ¿autenticada pero no conectada? se puede escribir…
    usuario: String,       // ¿y qué usuario hay si no está autenticada? ""
}

// 2. BIEN: un enum con un caso por estado; cada caso lleva solo sus datos.
#[derive(Debug)]
enum Conexion {
    Desconectada,
    Conectada { intentos: u32 },
    Autenticada { usuario: String },
}

impl Conexion {
    fn conectar(self) -> Conexion {
        match self {
            Conexion::Desconectada => Conexion::Conectada { intentos: 0 },
            otra => otra, // ya estaba conectada: no cambia
        }
    }

    fn entrar(self, usuario: &str, clave: &str) -> Conexion {
        match self {
            Conexion::Conectada { intentos } if clave == "abracadabra" => {
                println!("  (entró al intento {})", intentos + 1);
                Conexion::Autenticada { usuario: usuario.to_string() }
            }
            Conexion::Conectada { intentos } => Conexion::Conectada { intentos: intentos + 1 },
            otra => otra,
        }
    }
}

// 3. Newtype con constructor que valida: una Nota solo puede valer 0..=10.
#[derive(Debug)]
struct Nota(u8);

impl Nota {
    fn new(valor: u8) -> Option<Nota> {
        if valor <= 10 { Some(Nota(valor)) } else { None }
    }
}

fn main() {
    let c = Conexion::Desconectada;
    let c = c.conectar();
    let c = c.entrar("lucía", "1234");
    println!("{:?}", c);
    let c = c.entrar("lucía", "abracadabra");
    println!("{:?}", c);

    println!("{:?} {:?}", Nota::new(9), Nota::new(42));
}
