const TABLA: [(u32, &str); 13] = [
    (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"), (90, "XC"),
    (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
];

fn a_romano(mut n: u32) -> String {
    let mut s = String::new();
    for (valor, simbolo) in TABLA {
        while n >= valor {
            s.push_str(simbolo);
            n -= valor;
        }
    }
    s
}

fn de_romano(s: &str) -> Option<u32> {
    let valor = |c: char| match c {
        'I' => Some(1), 'V' => Some(5), 'X' => Some(10), 'L' => Some(50),
        'C' => Some(100), 'D' => Some(500), 'M' => Some(1000), _ => None,
    };
    let v: Vec<u32> = s.chars().map(valor).collect::<Option<_>>()?;
    let mut total = 0;
    for i in 0..v.len() {
        if i + 1 < v.len() && v[i] < v[i + 1] { total -= v[i] as i64 } else { total += v[i] as i64 }
    }
    let total = u32::try_from(total).ok().filter(|t| (1..=3999).contains(t))?;
    (a_romano(total) == s).then_some(total)
}
