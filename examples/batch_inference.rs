use litert_lm::{Backend, Engine};

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

    // Create engine
    let engine = Engine::new(model_path, Backend::Cpu)?;
    println!("Engine created successfully!\n");

    // Test prompts
    let prompts = vec![
        "What is the capital of France?",
        "Explain quantum computing in simple terms.",
        "Write a haiku about programming.",
        "What is 2 + 2?",
    ];

    println!("Running batch inference...\n");
    println!("========================================");

    // Process each prompt in a separate session
    for (i, prompt) in prompts.iter().enumerate() {
        println!("\n[{}] Prompt: {}", i + 1, prompt);

        // Create a new session for each prompt
        let session = engine.create_session()?;

        match session.generate(prompt) {
            Ok(response) => {
                println!("Response: {}", response);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        println!("----------------------------------------");
    }

    println!("\nBatch inference complete!");

    Ok(())
}
