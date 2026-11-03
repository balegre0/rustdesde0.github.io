fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    // adaptadores: perezosos, devuelven otro iterador
    let dobles: Vec<i32> = v.iter().map(|x| x * 2).collect();
    let pares: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("{dobles:?} {pares:?}");
    for (i, x) in v.iter().enumerate().skip(5) {
        print!("{i}:{x} ");
    }
    println!();
    let nombres = ["Ana", "Luis", "Eva"];
    let edades = [31, 25, 40];
    let pares: Vec<(&str, i32)> = nombres.iter().copied().zip(edades).collect();
    println!("{pares:?}");
    let todo: Vec<i32> = [1, 2].into_iter().chain([3, 4]).rev().collect();
    println!("{todo:?}");
    let primeros: Vec<i32> = (1..).step_by(3).take(4).collect();
    println!("{primeros:?}");
    let letras: Vec<char> = ["hola", "sí"].iter().flat_map(|s| s.chars()).collect();
    println!("{letras:?}");
    let numeros: Vec<i32> = ["1", "x", "3"].iter().filter_map(|s| s.parse().ok()).collect();
    println!("{numeros:?}");
    let mut p = [1, 2, 3].into_iter().peekable();
    let mira = p.peek().copied(); // peek mira el siguiente sin sacarlo
    let saca = p.next();
    println!("{mira:?} {saca:?} {:?}", p.peek());
    let hasta: Vec<i32> = v.iter().copied().take_while(|x| *x < 5).collect();
    println!("{hasta:?}");
    let ventanas: Vec<i32> = v.windows(2).map(|w| w[0] + w[1]).collect();
    println!("{ventanas:?}");
}
