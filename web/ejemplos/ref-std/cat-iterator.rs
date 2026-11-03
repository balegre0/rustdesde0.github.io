use std::iter;

fn main() {
    let v = [3, 1, 4, 1, 5, 9, 2, 6];

    // scan: un acumulador que se emite en cada paso (sumas parciales).
    let parciales: Vec<i32> = v.iter().scan(0, |s, &x| { *s += x; Some(*s) }).collect();
    println!("scan:        {parciales:?}");

    // map_while: transforma mientras la closure devuelva Some.
    let hasta_el_cero: Vec<u32> = ["7", "8", "x", "9"].iter().map_while(|s| s.parse().ok()).collect();
    println!("map_while:   {hasta_el_cero:?}");

    // reduce: como fold, pero empieza con el primer elemento (None si está vacío).
    println!("reduce:      {:?}", v.iter().copied().reduce(i32::max));

    // try_fold: un fold que puede fallar a mitad (aquí, si la suma desborda un u8).
    let suma: Option<u8> = [100u8, 100, 100].iter().try_fold(0u8, |a, &x| a.checked_add(x));
    println!("try_fold:    {suma:?}");

    // is_sorted, position, rposition, find_map.
    println!("is_sorted:   {}", v.is_sorted());
    println!("position/rposition del 1: {:?} {:?}", v.iter().position(|&x| x == 1), v.iter().rposition(|&x| x == 1));
    println!("find_map:    {:?}", ["a", "12", "b"].iter().find_map(|s| s.parse::<i32>().ok()));

    // partition y unzip: dos colecciones de una pasada.
    let (pares, impares): (Vec<i32>, Vec<i32>) = v.iter().partition(|&&x| x % 2 == 0);
    let (letras, numeros): (Vec<char>, Vec<i32>) = [('a', 1), ('b', 2)].into_iter().unzip();
    println!("partition:   {pares:?} {impares:?}   unzip: {letras:?} {numeros:?}");

    // Comparar iteradores directamente (orden lexicográfico).
    println!("cmp:         {:?}", [1, 2, 3].iter().cmp([1, 2, 4].iter()));

    // Fuentes: successors, from_fn, repeat_n, once, zip libre.
    let potencias: Vec<u32> = iter::successors(Some(1u32), |x| x.checked_mul(10)).collect();
    println!("successors:  {potencias:?}");
    let mut n = 0;
    let contador: Vec<i32> = iter::from_fn(|| { n += 1; (n <= 3).then_some(n) }).collect();
    println!("from_fn:     {contador:?}");
    let s: String = iter::repeat_n('ñ', 3).chain(iter::once('!')).collect();
    println!("repeat_n:    {s}");
    let sumas: Vec<i32> = iter::zip([1, 2, 3], [10, 20, 30]).map(|(a, b)| a + b).collect();
    println!("iter::zip:   {sumas:?}");

    // cycle + take, step_by, inspect.
    let ciclo: String = "ab".chars().cycle().take(5).collect();
    println!("cycle:       {ciclo}");
    let total: i32 = (0..10).step_by(3).inspect(|x| print!("[{x}]")).sum();
    println!(" -> suma {total}");
}
