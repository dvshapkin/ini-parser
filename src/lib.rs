use std::collections::HashMap;
use std::str::FromStr;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
