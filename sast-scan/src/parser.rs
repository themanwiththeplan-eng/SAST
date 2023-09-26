pub mod parser {
        pub fn parse(file: String){
        let mut file = std::fs::File::open(&file).unwrap();
        let mut buff = String::new();

        std::io::Read::read_to_string(&mut file, &mut buff).unwrap();
        let _config: String = serde_json::from_str(&mut buff).unwrap();
        println!("{}", _config);
    }
}