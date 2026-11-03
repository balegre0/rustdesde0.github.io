fn main() {
    let textos = ["4", "x", "6"];
    let todos: Result<Vec<i32>, _> = textos.iter().map(|t| t.parse::<i32>()).collect();
    println!("{:?}", todos);
    let buenos: Vec<i32> = textos.iter().filter_map(|t| t.parse().ok()).collect();
    println!("{:?}", buenos);
}
