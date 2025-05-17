use std::io;

fn main() {
    let mut dict = Vec::new();

    loop {
        let mut user_input = String::new();

        println!("Escribe una tarea:");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Something went wrong");

        let trimmed = user_input.trim();
        dict.push(trimmed.to_string());

        if trimmed == "exit" {
            break;
        }

        for (i, v) in dict.iter().enumerate() {
            println!("{} . {}", i, v);
        }
    }
}
