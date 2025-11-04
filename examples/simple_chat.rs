use litert_lm::{Backend, Engine};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get model path from command line argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <model_path>", args[0]);
        eprintln!("Example: {} model.tflite", args[0]);
        std::process::exit(1);
    }
    let model_path = &args[1];

    println!("Loading model from: {}", model_path);

    // Create engine with CPU backend
    let engine = Engine::new(model_path, Backend::Cpu)?;
    println!("Engine created successfully!");

    // Create a session (conversation)
    let session = engine.create_session()?;
    println!("Session created successfully!");
    println!();
    println!("You can now chat with the model. Type 'quit' or 'exit' to stop.");
    println!("========================================");
    println!();

    // Interactive chat loop
    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // Generate response
        print!("Assistant: ");
        io::stdout().flush()?;

        match session.generate(input) {
            Ok(response) => {
                println!("{}", response);
                println!();
            }
            Err(e) => {
                eprintln!("Error generating response: {}", e);
                println!();
            }
        }
    }

    Ok(())
}
