// JEFE DEL MUNDO 9: la máquina expendedora (solución).

#[derive(Debug, Clone, Copy)]
enum Moneda { Cent10, Cent20, Cent50, Euro1 }

impl Moneda {
    fn centimos(self) -> u32 {
        match self {
            Moneda::Cent10 => 10,
            Moneda::Cent20 => 20,
            Moneda::Cent50 => 50,
            Moneda::Euro1 => 100,
        }
    }
}

enum Accion {
    Meter(Moneda),
    Pedir(usize),
    Devolver,
}

struct Producto {
    nombre: &'static str,
    precio: u32,
}

struct Maquina {
    saldo: u32,
    productos: Vec<Producto>,
}

impl Maquina {
    fn new() -> Self {
        Maquina {
            saldo: 0,
            productos: vec![
                Producto { nombre: "agua", precio: 80 },
                Producto { nombre: "zumo", precio: 150 },
            ],
        }
    }

    fn procesar(&mut self, accion: Accion) -> Option<String> {
        match accion {
            Accion::Meter(moneda) => {
                self.saldo += moneda.centimos();
                None
            }
            Accion::Pedir(n) => {
                let Some(p) = self.productos.get(n) else {
                    return Some(format!("no existe el producto {n}"));
                };
                if self.saldo < p.precio {
                    return Some(format!("faltan {} céntimos", p.precio - self.saldo));
                }
                self.saldo -= p.precio;
                Some(format!("toma tu {}", p.nombre))
            }
            Accion::Devolver => match self.saldo {
                0 => None,
                s => {
                    self.saldo = 0;
                    Some(format!("devuelvo {s} céntimos"))
                }
            },
        }
    }
}

fn main() {
    let mut m = Maquina::new();
    let acciones = [
        Accion::Pedir(0),
        Accion::Meter(Moneda::Cent50),
        Accion::Meter(Moneda::Cent20),
        Accion::Pedir(0),
        Accion::Meter(Moneda::Cent10),
        Accion::Pedir(0),
        Accion::Pedir(7),
        Accion::Meter(Moneda::Euro1),
        Accion::Meter(Moneda::Cent20),
        Accion::Devolver,
        Accion::Devolver,
    ];
    for a in acciones {
        if let Some(texto) = m.procesar(a) {
            println!("{texto}");
        }
    }
    println!("saldo final: {}", m.saldo);
}
