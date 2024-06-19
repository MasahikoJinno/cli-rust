fn main() {
    match catr::get_args() {
        Ok(config) => {
            if let Err(err) = catr::run(config) {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
