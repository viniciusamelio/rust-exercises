// 1. Cada valor em Rust tem um owner
// 2. Só pode haver um owner por vez
// 3. Quando o owner sai do escopo, o valor é liberado

#[derive(Clone, Copy)]
enum Status {
    Offline,
    Online,
}

fn main() {
    // clone - will make a deep copy of the value, keeping the original intact
    let initial_status = Status::Offline;
    let _current_status = initial_status.clone();
    println!("status: {}", status_to_string(&initial_status));

    // borrowing - will reference the value without taking ownership
    let initial_status = Status::Offline;
    let _current_status = &initial_status;
    println!("status: {}", status_to_string(_current_status));

    // mut reference - will allow us to change the value
    let mut _current_status = initial_status;
    change_status(&mut _current_status);
    println!("status: {}", status_to_string(&_current_status));
}

fn change_status(status: &mut Status) {
    *status = Status::Online;
}

fn status_to_string(status: &Status) -> String {
    match status {
        Status::Offline => String::from("offline"),
        Status::Online => String::from("online"),
    }
}
