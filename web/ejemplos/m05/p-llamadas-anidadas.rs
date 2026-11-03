fn mas_uno(n: i32) -> i32 {
    n + 1
}

fn por_dos(n: i32) -> i32 {
    n * 2
}

fn main() {
    println!("{}", por_dos(mas_uno(por_dos(3))));
}
