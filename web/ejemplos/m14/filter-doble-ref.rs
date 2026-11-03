fn main() {
    let v = vec![1, 5, 2, 7];
    let grandes: Vec<&i32> = v.iter().filter(|x| x > 3).collect();
    println!("{grandes:?}");
}
