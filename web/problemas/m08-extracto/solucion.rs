struct Extracto<'a> {
    texto: &'a str,
}

impl<'a> Extracto<'a> {
    fn primera_frase(&self) -> &'a str {
        match self.texto.find('.') {
            Some(i) => &self.texto[..i],
            None => self.texto,
        }
    }

    fn palabras(&self) -> usize {
        self.texto.split_whitespace().count()
    }
}
