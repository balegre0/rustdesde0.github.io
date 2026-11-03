#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Empuja(i64),
    Suma,
    Resta,
    Mul,
    Div,
}

fn sacar(pila: &mut Vec<i64>) -> Result<i64, String> {
    pila.pop().ok_or_else(|| "pila vacía".to_string())
}

pub fn ejecutar(prog: &[Instr]) -> Result<i64, String> {
    let mut pila: Vec<i64> = Vec::new();
    for &i in prog {
        match i {
            Instr::Empuja(n) => pila.push(n),
            op => {
                let b = sacar(&mut pila)?;
                let a = sacar(&mut pila)?;
                let r = match op {
                    Instr::Suma => a.checked_add(b),
                    Instr::Resta => a.checked_sub(b),
                    Instr::Mul => a.checked_mul(b),
                    Instr::Div => {
                        if b == 0 { return Err("división por cero".to_string()); }
                        a.checked_div(b)
                    }
                    Instr::Empuja(_) => unreachable!(),
                };
                pila.push(r.ok_or_else(|| "desbordamiento".to_string())?);
            }
        }
    }
    if pila.len() != 1 {
        return Err(format!("la pila termina con {} valores", pila.len()));
    }
    Ok(pila[0])
}
