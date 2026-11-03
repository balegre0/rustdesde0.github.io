// 1. Un «iterador que presta»: cada elemento es un préstamo de `self`.
//    `Item<'a>` es un tipo asociado GENÉRICO (GAT): depende de 'a.
trait IteradorQuePresta {
    type Item<'a>
    where
        Self: 'a;

    fn siguiente(&mut self) -> Option<Self::Item<'_>>;
}

// 2. Ventanas EXCLUSIVAS (&mut) de 2 elementos sobre un slice.
struct VentanasMut<'s> {
    datos: &'s mut [i32],
    pos: usize,
}

impl<'s> IteradorQuePresta for VentanasMut<'s> {
    type Item<'a>
        = &'a mut [i32]
    where
        Self: 'a;

    fn siguiente(&mut self) -> Option<&mut [i32]> {
        let inicio = self.pos;
        self.pos += 1;
        self.datos.get_mut(inicio..inicio + 2)
    }
}

fn main() {
    let mut v = [1, 2, 3, 4];
    let mut it = VentanasMut { datos: &mut v, pos: 0 };
    // 3. Cada ventana se puede modificar; las ventanas se solapan,
    //    pero nunca hay dos vivas a la vez.
    while let Some(ventana) = it.siguiente() {
        ventana[1] += ventana[0];
        println!("{ventana:?}");
    }
    println!("{v:?}");
}
