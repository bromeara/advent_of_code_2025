pub mod util {
    pub fn parse_args(args: &[String]) -> Result<&str, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }
        let file_path = &args[1];
        println!("Loading: {file_path}");
        Ok(file_path)
    }
}
