struct Parser<T, U> {
    f: Fn(Vec<T>) -> (U, Vec<T>),
}

impl <T, U> Parser<T, U> {
    pub fn parse(&self, input: Vec<T>) -> Result<U, String> {
        let (value, remaining) = (self.f)(input);
        if remaining.len() > 0 {
            Result::Err(String::from("failed"))
        } else {
            Result::Ok(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}