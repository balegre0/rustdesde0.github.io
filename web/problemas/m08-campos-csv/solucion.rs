struct Campos<'a> {
    resto: Option<&'a str>,
}

impl<'a> Campos<'a> {
    fn siguiente(&mut self) -> Option<&'a str> {
        let texto = self.resto?;
        match texto.find(',') {
            Some(i) => {
                self.resto = Some(&texto[i + 1..]);
                Some(&texto[..i])
            }
            None => {
                self.resto = None;
                Some(texto)
            }
        }
    }
}
