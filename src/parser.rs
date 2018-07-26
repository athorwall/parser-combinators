use std::clone::Clone;

type ParseFunc<T, U> = Fn(&Vec<T>) -> Vec<(U, Vec<T>)>;

struct Parser<T, U> {
    f: Box<ParseFunc<T, U>>,
}

impl <T, U> Parser<T, U> {
    pub fn parse(&self, input: Vec<T>) -> Result<U, String> {
        let mut results = (self.f)(&input);
        let head = results.pop();
        match head {
            Some(first_result) => {
                let (value, remaining) = first_result;
                if remaining.len() > 0 {
                    Result::Err(String::from("failed"))
                } else {
                    Result::Ok(value)
                }
            },
            None => Result::Err(String::from("failed")),
        }
    }

    pub fn one(t: T) -> Parser<T, T> where T: Copy {
        Parser{
            f: Box::from(|input: &Vec<T>| {
                match input.first() {
                    Some(value) => {
                        let mut new_input = input.clone();
                        let first = new_input.pop().unwrap();
                        vec![(first, new_input)]
                    },
                    None => {
                        vec![]
                    }
                }
            }),
        }
    }

    pub fn or<V, W>(p1: &Parser<V, W>, p2: &Parser<V, W>) -> Parser<V, W> where V: Copy, W: Copy {
        // ???
        let cloned_p1 = p1.clone();
        let cloned_p2 = p2.clone();
        Parser{
            f: Box::from(move |input: &Vec<V>| {
                let mut p1_results = (cloned_p1.f)(input);
                let mut results = (cloned_p2.f)(input);
                results.append(&mut p1_results);
                results
            })
        }

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let parser = Parser::<char, char>::one('a');
        assert_eq!(parser.parse(vec!['a']), Result::Ok('a'));
        assert!(parser.parse(vec!['a', 'b']).is_err());
        assert!(parser.parse(vec![]).is_err());
    }

    #[test]
    fn test_or() {
        let parser_a = Parser::<char, char>::one('a');
        let parser_b = Parser::<char, char>::one('b');
        let parser = Parser::or(&parser_a, &parser_b);
        assert_eq!(parser.parse(vec!['a']), Result::Ok('a'));
        assert_eq!(parser.parse(vec!['b']), Result::Ok('b'));
    }
}