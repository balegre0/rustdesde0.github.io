fn main() {
    let invitados = vec![String::from("Ana"), String::from("Luis")];
    // Una vista prestada: el Vec sigue siendo el dueño
    let primero = &invitados[0];
    println!("{primero}");
}
