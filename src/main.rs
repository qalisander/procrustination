fn main() {
    route_parent(0)
}

fn route_parent(selector: u32) {
    #[deny(unreachable_patterns)]
    match selector {
        0 => {
            println!("route0 - 0")
        }
        1 => {
            println!("route0 - 1")
        }
        2 => {
            println!("route0 - 2")
        }
        3 => {
            println!("route0 - 3")
        }
        selector => route_child(selector),
    }
}

fn route_child(selector: u32) {
    #[deny(unreachable_patterns)]
    match selector {
        0 => {
            println!("route1 - 0")
        }
        1 => {
            println!("route1 - 1")
        }
        2 => {
            println!("route1 - 2")
        }
        3 => {
            println!("route1 - 3")
        }
        _ => {}
    }
}
