pub mod parser {
        pub fn parse(){
        let mut file = std::fs::File::open("../config.json").unwrap();
        let mut buff = String::new();

        std::io::Read::read_to_string(&mut file, &mut buff).unwrap();
        let _config: String = serde_json::from_str(&mut buff).unwrap();
    }
}