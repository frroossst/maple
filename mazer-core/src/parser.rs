#[derive(Debug, Clone, PartialEq)]
pub enum LispFragments {
    OpenParen,
    ClosedParen,
    Symbol(String),
    Number(f64),
}

pub enum ASTNode {
    
}

pub struct Parser {
    tokens: Vec<LispFragments>,
    cursor: usize,
}

impl Parser {
    pub fn new(src: String) -> Self {
        let tokens: Vec<LispFragments> = src
            .replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(|t| {
                match t {
                    "(" => LispFragments::OpenParen,
                    ")" => LispFragments::ClosedParen,
                    _ => {
                        if let Ok(num) = t.parse::<f64>() {
                            return LispFragments::Number(num);
                        } else {
                            return LispFragments::Symbol(t.to_string());
                        }
                    }
                }
            }).collect::<Vec<LispFragments>>();

        Parser {
            tokens,
            cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let ast: Vec<ASTNode> = Vec::new();

        // check balanced parenthesis
        let mut count = 0;
        let _ = self.tokens.clone().into_iter().map(|item| {
            match item {
                LispFragments::ClosedParen => count -= 1,
                LispFragments::OpenParen => count += 1,
                _ => {},
            }
        });


        ast
    }
}

