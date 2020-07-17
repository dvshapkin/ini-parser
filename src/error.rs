use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug)]
pub struct ParseError {
    msg: String
}

impl ParseError {
    pub fn new(msg: &str) -> Self {
        ParseError {
            msg: msg.to_owned()
        }
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> { 
        f.write_str(&self.msg)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ok() {
        let e = ParseError::new("Parse error");
        assert_eq!("Parse error", format!("{}", e));
    }
}