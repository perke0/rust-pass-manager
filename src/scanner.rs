pub fn get_input() -> String {
    use std::io;
    println!("Type password here: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input from user");

    input
}
