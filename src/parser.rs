use std::clone::Clone;

struct Parser<'a, T> {
    f: Box<Fn(&String) -> Vec<(T, String)> + 'a>,
}

impl <'a, T> Parser<'a, T> {
    pub fn parse(&self, input: &String) -> Result<T, String> {
        let mut results = (self.f)(&input);
        let head = results.pop();
        match head {
            Some(first_result) => {
                let (value, remaining) = first_result;
                if remaining.len() > 0 {
                    Result::Err(String::from("ambiguous"))
                } else {
                    Result::Ok(value)
                }
            },
            None => Result::Err(String::from("no parse")),
        }
    }

    pub fn map<U>(self, func: Box<Fn(&T) -> U>) -> Parser<'a, U> where T: 'a, U: 'a {
        Parser{
            f: Box::from(move |input: &String| {
                let results = (self.f)(input);
                return results.iter()
                    .map(|(u, rest)| {
                        (func(u), rest.clone())
                    })
                    .collect();
            })
        }
    }

    // Returns a parser that parses the character in t.
    pub fn one(t: char) -> Parser<'a, char> {
        Parser{
            f: Box::from(move |input: &String| {
                match input.chars().next() {
                    Some(value) => {
                        let mut new_input = input.clone();
                        let mut chars = input.chars();
                        let first = chars.next().unwrap();
                        let remaining = chars.collect();
                        if first == t {
                            vec![(first, remaining)]
                        } else {
                            vec![]
                        }
                    },
                    None => {
                        vec![]
                    }
                }
            }),
        }
    }

    // Returns a parser that runs both p1 and p2, and combines the results.
    pub fn combine<'b, U>(p1: Parser<'b, U>, p2: Parser<'b, U>) -> Parser<'b, U>
        where U: Copy + 'b {
        Parser{
            f: Box::from(move |input: &String| {
                let mut p1_results = (p1.f)(input);
                let mut results = (p2.f)(input);
                results.append(&mut p1_results);
                results
            })
        }
    }

    // Returns a parser that runs p1, and then runs p2 if p1 fails.
    pub fn option<'b, U>(p1: Parser<'b, U>, p2: Parser<'b, U>) -> Parser<'b, U>
        where U: Copy + 'b {
        Parser{
            f: Box::from(move |input: &String| {
                let p1_results = (p1.f)(input);
                if p1_results.is_empty() {
                    (p2.f)(input)
                } else {
                    p1_results
                }
            })
        }
    }

    // Returns a parser that results from composing p2 monadically over p1.
    pub fn bind<'b, U, V>(p1: Parser<'b, U>, p2: Box<Fn(U) -> Parser<'b, V>>) -> Parser<'b, V>
        where U: Copy + 'b, V: 'b {
        Parser{
            f: Box::from(move |input: &String| {
                (p1.f)(input).iter()
                    .flat_map(|(value, remaining)| {
                        (p2(*value).f)(remaining)
                    })
                    .collect()
            })
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let parser = Parser::<char>::one('a')
            .map(Box::from(|c: &char| c.to_string()));
        assert_eq!(parser.parse(&String::from("a")), Result::Ok(String::from("a")));
    }

    #[test]
    fn test_one() {
        let parser = Parser::<char>::one('a');
        assert_eq!(parser.parse(&String::from("a")), Result::Ok('a'));
        assert!(parser.parse(&String::from("ab")).is_err());
        assert!(parser.parse(&String::from("")).is_err());
    }

    #[test]
    fn test_combine() {
        let parser_a = Parser::<char>::one('a');
        let parser_b = Parser::<char>::one('b');
        let parser = Parser::<char>::combine(parser_a, parser_b);
        assert_eq!(parser.parse(&String::from("a")), Result::Ok('a'));
        assert_eq!(parser.parse(&String::from("b")), Result::Ok('b'));
        assert!(parser.parse(&String::from("c")).is_err());
        assert!(parser.parse(&String::from("ab")).is_err());
        assert!(parser.parse(&String::from("")).is_err());
    }

    #[test]
    fn test_option() {
        let parser_a = Parser::<char>::one('a');
        let parser_b = Parser::<char>::one('b');
        let parser = Parser::<char>::option(parser_a, parser_b);
        assert_eq!(parser.parse(&String::from("a")), Result::Ok('a'));
        assert_eq!(parser.parse(&String::from("b")), Result::Ok('b'));
        assert!(parser.parse(&String::from("c")).is_err());
        assert!(parser.parse(&String::from("ab")).is_err());
        assert!(parser.parse(&String::from("")).is_err());
    }

    #[test]
    fn test_bind() {
        let p1 = Parser::<char>::one('a');
        let f = |a: char| {
            Parser::<char>::one('b').map(Box::from(move |b: &char| {
                let mut s = a.to_string();
                s + b.to_string().as_str()
            }))
        };
        let p = Parser::<char>::bind(p1, Box::from(f));
        assert_eq!(p.parse(&String::from("ab")), Result::Ok(String::from("ab")));
    }
}