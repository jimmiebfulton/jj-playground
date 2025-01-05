mod messages;

fn main() {
    print_message(&messages::create_message());
    print_departure_message();
}

fn print_message(message: &str) {
    println!("{message}");
}

fn print_departure_message() {
    println!("Goodbye, world!");
}
