fn doble(x: i32) -> i32 {
    x * 2
}

#[cfg(test)]
mod tests {
    pub fn caso_basico() -> bool {
        super::doble(2) == 4
    }
}

fn main() {
    // En un build normal (sin --test), el módulo tests no existe.
    println!("{}", tests::caso_basico());
}
