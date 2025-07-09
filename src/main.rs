use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();  // Load environment variables from .env file
    let api_key = env::var("OPENAI_API_KEY").expect("Key not set in .env file");

    print!("api key: {}", api_key);


    
}
