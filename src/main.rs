mod cli;
mod logger;
mod template;






fn main() {
    logger::init_logger();
    cli::run_cli();
    println!("{}",template::list_templates()[0]);
    
}



