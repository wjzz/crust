use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    KEYWORD(String),
    IDENTIFIER(String),
}

struct TokenizerState {
    i: usize,
    n: usize,
    chars: Vec<char>,
    ungetch: Option<char>,
}

impl TokenizerState {
    fn new(input: &str) -> TokenizerState {
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        let n = chars.len();
        let ungetch = None;
        TokenizerState {
            chars,
            i,
            n,
            ungetch,
        }
    }

    fn ungetc(&mut self, chr: char) {
        assert_eq!(self.ungetch, None);
        self.ungetch = Some(chr);
    }

    fn get_char(&mut self) -> Option<char> {
        if let Some(chr) = self.ungetch {
            self.ungetch = None;
            Some(chr)
        } else if self.i < self.n {
            let chr = self.chars[self.i];
            self.i += 1;
            Some(chr)
        } else {
            None
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        let res = self.get_char();
        self.i -= 1;
        res
    }

    fn tokenize_identfier(&mut self) -> String {
        let mut ident = String::new();
        loop {
            if let Some(chr) = self.get_char() {
                if chr.is_alphabetic() || chr.is_ascii_digit() || chr == '_' {
                    println!("chr = {}", chr);
                    ident.push(chr);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        ident
    }

    fn tokenize_iter(&mut self) -> Vec<Token> {
        let charmap: HashMap<char, Token> = HashMap::from([
            ('(', Token::LPAREN),
            (')', Token::RPAREN),
            ('{', Token::LBRACE),
            ('}', Token::RBRACE),
        ]);

        let keyword_set: HashSet<&'static str> =
            HashSet::from(["int", "long", "unsigned", "short", "struct"]);

        let mut tokens = vec![];

        while let Some(chr) = self.get_char() {
            if chr.is_whitespace() {
                continue;
            } else if chr.is_alphabetic() || chr == '_' {
                self.ungetc(chr);
                let ident = self.tokenize_identfier();
                if keyword_set.contains(&*ident) {
                    tokens.push(Token::KEYWORD(ident));
                } else {
                    tokens.push(Token::IDENTIFIER(ident));
                }
            } else if let Some(res) = charmap.get(&chr) {
                tokens.push(res.clone());
            } else {
                panic!("Unknown character!: {}", chr);
            }
        }

        // TODO: add the EOF token

        tokens
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    TokenizerState::new(input).tokenize_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn parens_braces() {
        let v1 = vec![LBRACE, RBRACE, LPAREN, RPAREN];
        let res = tokenize("{} (\n )\t");
        assert_eq!(v1, res);
    }

    #[test]
    fn identifiers() {
        let ids = vec!["x", "abc", "hello123", "_foo", "__gcc_test", "int66"];
        for identifier in ids {
            let res = tokenize(identifier);
            assert_eq!(res, vec![IDENTIFIER(String::from(identifier))]);
        }
    }

    #[test]
    fn keywords() {
        let keywords = vec!["unsigned", "short", "struct", "int", "long"];
        for keyword in keywords {
            let res = tokenize(keyword);
            assert_eq!(res, vec![KEYWORD(String::from(keyword))]);
        }
    }
}
