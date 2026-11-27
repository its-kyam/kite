use std::io::stdout as get_stdout;
use std::io::{Write, stdin as get_stdin};

const DEFAULT_AGENT: &str = "Qwen";

const EXIT_COMMAND: &str = "/quit";

fn bye() {
    println!("Bye bye!");
}

fn main() {
    println!("Let's work with the agent!");

    let stdin = get_stdin();
    let mut stdout = get_stdout();

    println!("Type /quit to exit");

    loop {
        let mut input = String::new();

        print!("> ");
        stdout.flush().unwrap();
        if stdin.read_line(&mut input).expect("Failed to read line") == 0 {
            return bye();
        };

        let input = input.trim();

        if input == EXIT_COMMAND {
            return bye();
        }

        println!("{DEFAULT_AGENT}: {input}");
    }
}
