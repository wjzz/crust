use std::collections::{HashMap, HashSet};

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    L_SQUARE_BRACKET,
    R_SQUARE_BRACKET,
    KEYWORD(String),
    IDENTIFIER(String),
    NUMBER(u64),
    B_AND,
    B_OR,
    EQUAL,
    PLUS,
    MINUS,
    TIMES,
    SLASH,
    PERCENT,
    AMPERSAND,
    LESS,
    GREATER,
    VERTICAL_BAR,
    XOR,
    SHL,
    SHR,
    BANG,
    TILDE,
    COMMA,
    DOT,
    QUESTION_MARK,
    QUOTE,
    DOUBLE_QUOTE,
    COLON,
    SEMICOLON,
    EOF,
}

struct TokenizerState {
    i: usize,
    n: usize,
    chars: Vec<char>,
    ungetch: Option<char>,
}

impl TokenizerState {
    fn new(input: &str) -> TokenizerState {
        let mut chars: Vec<char> = input.chars().collect();
        chars.push(' ');
        let i = 0;
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
        while let Some(chr) = self.get_char() {
            if chr.is_alphabetic() || chr.is_ascii_digit() || chr == '_' {
                println!("chr = {}", chr);
                ident.push(chr);
            } else {
                break;
            }
        }
        ident
    }

    fn tokenize_number(&mut self) -> u64 {
        let mut n = 0;
        while let Some(chr) = self.get_char() {
            if chr.is_ascii_digit() {
                println!("chr = {}", chr);
                n *= 10;
                n += chr.to_digit(10).unwrap() as u64;
            } else {
                break;
            }
        }
        n
    }

    fn tokenize_iter(&mut self) -> Vec<Token> {
        let charmap: HashMap<&'static str, Token> = HashMap::from([
            ("(", Token::LPAREN),
            (")", Token::RPAREN),
            ("{", Token::LBRACE),
            ("}", Token::RBRACE),
            ("[", Token::L_SQUARE_BRACKET),
            ("]", Token::R_SQUARE_BRACKET),
            ("+", Token::PLUS),
            ("-", Token::MINUS),
            ("*", Token::TIMES),
            ("/", Token::SLASH),
            ("%", Token::PERCENT),
            ("!", Token::BANG),
            ("^", Token::XOR),
            ("~", Token::TILDE),
            ("&", Token::AMPERSAND),
            ("|", Token::VERTICAL_BAR),
            ("&&", Token::B_AND),
            ("||", Token::B_OR),
            ("=", Token::EQUAL),
            (";", Token::SEMICOLON),
            (":", Token::COLON),
            ("?", Token::QUESTION_MARK),
            ("'", Token::QUOTE),
            ("\"", Token::DOUBLE_QUOTE),
        ]);

        let keyword_set: HashSet<&'static str> =
            HashSet::from(["int", "long", "unsigned", "short", "struct"]);

        let mut tokens = vec![];

        while let Some(chr) = self.get_char() {
            if chr.is_whitespace() {
                continue;
            } else if chr.is_digit(10) {
                self.ungetc(chr);
                let n = self.tokenize_number();
                tokens.push(Token::NUMBER(n));
            } else if chr.is_alphabetic() || chr == '_' {
                self.ungetc(chr);
                let ident = self.tokenize_identfier();
                if keyword_set.contains(&*ident) {
                    tokens.push(Token::KEYWORD(ident));
                } else {
                    tokens.push(Token::IDENTIFIER(ident));
                }
            } else {
                // check for multi-character operators (like '&&' or '+=')
                // TODO: support triple-character ops (like '<<=' or '>>=')
                let chr2 = self.get_char().unwrap();
                let s = String::from_iter(vec![chr, chr2]);
                if let Some(res) = charmap.get(&*s) {
                    tokens.push(res.clone());
                } else {
                    if let Some(res) = charmap.get(&*String::from(chr)) {
                        tokens.push(res.clone());
                        self.ungetc(chr2);
                    } else {
                        panic!("Unknown character!: {}", chr);
                    }
                }
            }
        }

        tokens.push(Token::EOF);
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
        let v1 = vec![LBRACE, RBRACE, LPAREN, L_SQUARE_BRACKET, R_SQUARE_BRACKET, RPAREN, EOF];
        let res = tokenize("{} (\n [] )\t");
        assert_eq!(v1, res);
    }

    #[test]
    fn identifiers() {
        let ids = vec!["x", "abc", "hello123", "_foo", "__gcc_test", "int66"];
        for identifier in ids {
            let res = tokenize(identifier);
            assert_eq!(res, vec![IDENTIFIER(String::from(identifier)), EOF]);
        }
    }

    #[test]
    fn keywords() {
        let keywords = vec!["unsigned", "short", "struct", "int", "long"];
        for keyword in keywords {
            let res = tokenize(keyword);
            assert_eq!(res, vec![KEYWORD(String::from(keyword)), EOF]);
        }
    }

    #[test]
    fn numbers() {
        let v1 = vec![NUMBER(123), NUMBER(0), NUMBER(33), EOF];
        let res = tokenize("123 0 33");
        assert_eq!(v1, res);
    }

    #[test]
    fn logical_ops() {
        let v1 = vec![B_AND, B_OR, BANG, EOF];
        let res = tokenize("&& || !");
        assert_eq!(v1, res);
    }

    #[test]
    fn arith_ops() {
        let v1 = vec![PLUS, MINUS, TIMES, SLASH, PERCENT, EOF];
        let res = tokenize("+ - * / %");
        assert_eq!(v1, res);
    }

    #[test]
    fn bitwise_ops() {
        let v1 = vec![AMPERSAND, VERTICAL_BAR, XOR, TILDE, EOF];
        let res = tokenize("& | ^ ~");
        assert_eq!(v1, res);
    }

    #[test]
    fn special_ops() {
        let v1 = vec![EQUAL, SEMICOLON, COLON, QUESTION_MARK, QUOTE, DOUBLE_QUOTE, EOF];
        let res = tokenize("= ; : ? ' \" ");
        assert_eq!(v1, res);
    }
}
