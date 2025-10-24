use std::path::Path;

fn main() {
    println!("Testing tokenizer loading...");

    let tokenizer_file = Path::new("models/codet5-small/tokenizer.json");
    println!("Checking file exists: {:?}", tokenizer_file.exists());

    if tokenizer_file.exists() {
        match tokenizers::Tokenizer::from_file(tokenizer_file) {
            Ok(tokenizer) => {
                println!("✅ Tokenizer loaded successfully");
                println!("Vocab size: {}", tokenizer.get_vocab_size(true));
            }
            Err(e) => {
                println!("❌ Failed to load tokenizer: {:?}", e);
            }
        }
    } else {
        println!("❌ Tokenizer file not found");
    }
}