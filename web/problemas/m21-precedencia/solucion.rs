use std::iter::Peekable;
use std::str::Chars;

struct Analizador<'a> {
    it: Peekable<Chars<'a>>,
}

impl Analizador<'_> {
    fn saltar_espacios(&mut self) {
        while self.it.peek().is_some_and(|c| c.is_whitespace()) {
            self.it.next();
        }
    }

    fn expresion(&mut self) -> Result<i64, String> {
        let mut v = self.termino()?;
        loop {
            self.saltar_espacios();
            match self.it.peek() {
                Some('+') => { self.it.next(); v += self.termino()?; }
                Some('-') => { self.it.next(); v -= self.termino()?; }
                _ => return Ok(v),
            }
        }
    }

    fn termino(&mut self) -> Result<i64, String> {
        let mut v = self.factor()?;
        loop {
            self.saltar_espacios();
            match self.it.peek() {
                Some('*') => { self.it.next(); v *= self.factor()?; }
                Some('/') => {
                    self.it.next();
                    let d = self.factor()?;
                    if d == 0 {
                        return Err(String::from("división entre cero"));
                    }
                    v /= d;
                }
                _ => return Ok(v),
            }
        }
    }

    fn factor(&mut self) -> Result<i64, String> {
        self.saltar_espacios();
        match self.it.peek().copied() {
            Some('(') => {
                self.it.next();
                let v = self.expresion()?;
                self.saltar_espacios();
                if self.it.next() != Some(')') {
                    return Err(String::from("falta un )"));
                }
                Ok(v)
            }
            Some(c) if c.is_ascii_digit() => {
                let mut n: i64 = 0;
                while let Some(d) = self.it.peek().and_then(|c| c.to_digit(10)) {
                    n = n * 10 + d as i64;
                    self.it.next();
                }
                Ok(n)
            }
            Some(c) => Err(format!("carácter inesperado: {c:?}")),
            None => Err(String::from("falta un número")),
        }
    }
}

fn evaluar(texto: &str) -> Result<i64, String> {
    let mut a = Analizador { it: texto.chars().peekable() };
    let v = a.expresion()?;
    a.saltar_espacios();
    match a.it.next() {
        None => Ok(v),
        Some(c) => Err(format!("sobra {c:?}")),
    }
}
