#[derive(Debug, Clone, Copy)]
pub enum Op { Local(usize), Const(i64) }

#[derive(Debug, Clone, Copy)]
pub enum Sentencia {
    Asigna(usize, Op),
    Suma(usize, Op, Op),
    Mul(usize, Op, Op),
}

#[derive(Debug, Clone)]
pub enum Terminador {
    Goto(usize),
    SwitchInt(usize, i64, usize, usize),
    Assert(usize, String),
    Return,
}

pub struct Bloque { pub sentencias: Vec<Sentencia>, pub terminador: Terminador }

pub fn ejecutar(bloques: &[Bloque], args: &[i64]) -> Result<i64, String> {
    let mut l = [0i64; 8];
    for (i, a) in args.iter().enumerate() {
        l[i + 1] = *a;
    }
    let val = |l: &[i64; 8], o: Op| match o { Op::Local(i) => l[i], Op::Const(c) => c };
    let mut actual = 0;
    for _ in 0..10_000 {
        let b = &bloques[actual];
        for s in &b.sentencias {
            match *s {
                Sentencia::Asigna(d, o) => l[d] = val(&l, o),
                Sentencia::Suma(d, x, y) => l[d] = val(&l, x) + val(&l, y),
                Sentencia::Mul(d, x, y) => l[d] = val(&l, x) * val(&l, y),
            }
        }
        actual = match &b.terminador {
            Terminador::Goto(n) => *n,
            Terminador::SwitchInt(x, v, si, no) => if l[*x] == *v { *si } else { *no },
            Terminador::Assert(x, msg) => {
                if l[*x] == 0 { return Err(msg.clone()); }
                actual + 1
            }
            Terminador::Return => return Ok(l[0]),
        };
    }
    Err("demasiados pasos".to_string())
}
