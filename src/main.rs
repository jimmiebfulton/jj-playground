mod cli;

fn main() {
    print_message(&cli::create_greeting());
    print_departure_message();
}

fn print_message(message: &str) {
    println!("{message}");
}

fn print_departure_message() {
    println!("Goodbye, world!");
}
