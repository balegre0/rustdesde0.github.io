macro_rules! rpn {
    // 1. Fin: queda un único valor en la pila
    (@ [$x:expr]) => { $x };
    // 2. Operadores: sacan dos valores (la cima es $b) y apilan el resultado
    (@ [$b:expr, $a:expr $(, $resto:expr)*] + $($t:tt)*) => { rpn!(@ [($a + $b) $(, $resto)*] $($t)*) };
    (@ [$b:expr, $a:expr $(, $resto:expr)*] - $($t:tt)*) => { rpn!(@ [($a - $b) $(, $resto)*] $($t)*) };
    (@ [$b:expr, $a:expr $(, $resto:expr)*] * $($t:tt)*) => { rpn!(@ [($a * $b) $(, $resto)*] $($t)*) };
    // 3. Un número: se apila
    (@ [$($pila:expr),*] $n:literal $($t:tt)*) => { rpn!(@ [$n $(, $pila)*] $($t)*) };
    // 4. Entrada
    ($($t:tt)+) => { rpn!(@ [] $($t)+) };
}
