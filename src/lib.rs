pub mod error;

use std::collections::HashMap;
use std::str::FromStr;

use error::ParseError;

#[derive(Debug)]
pub struct Config<Value: FromStr> {
    data: HashMap<String, HashMap<String, Box<Value>>>,
}

impl<Value: FromStr> Config<Value> {
    pub fn new() -> Self {
        Config {
            data: HashMap::new(),
        }
    }
}

impl<Value: FromStr> FromStr for Config<Value> {
    type Err = ParseError;
    fn from_str(data: &str) -> Result<Self, Self::Err> { 
        let data = data.trim();
        loop {
            if let Some(ch) = data.chars().next() {
                
            }
        }
        Ok(Config::new())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
