struct Extracto<'a> {
    texto: &'a str,
}

impl<'a> Extracto<'a> {
    fn primera_frase(&self) -> &'a str {
        todo!()
    }

    fn palabras(&self) -> usize {
        todo!()
    }
}
