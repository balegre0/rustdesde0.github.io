use std::collections::VecDeque;

fn atendidos(ordenes: &[&str]) -> (Vec<String>, Vec<String>) {
    let mut cola: VecDeque<String> = VecDeque::new();
    let mut hechos = Vec::new();
    for orden in ordenes {
        match orden.split_once(' ') {
            Some(("llega", n)) => cola.push_back(n.to_string()),
            Some(("urgente", n)) => cola.push_front(n.to_string()),
            _ if *orden == "atiende" => {
                if let Some(p) = cola.pop_front() {
                    hechos.push(p);
                }
            }
            _ => {}
        }
    }
    (hechos, cola.into_iter().collect())
}
