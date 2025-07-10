mod cli;
mod logger;







fn main() {
    logger::init_logger();
    cli::run_cli();
}



