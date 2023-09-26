mod parser;

use parser::parser::parse;
fn main(){ 
    parse("../config.json".to_owned());
}
