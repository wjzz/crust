use crate::lexer::Token;
use crate::ast::*;

#[derive(Debug)]
pub enum ParseError {
    NameExpected,
    NumberExpected,
    TypeExpected,
    Expected { expected_token: Token, token: Token },
}

struct ParseState {
    i: usize,
    tokens: Vec<Token>,
}

type Parser<T> = Result<T, ParseError>;

impl ParseState {
    fn new(tokens: Vec<Token>) -> ParseState {
        let i = 0;
        ParseState { i, tokens }
    }

    fn peek(&self) -> Token {
        return self.tokens[self.i].clone();
    }

    fn skip(&mut self) {
        self.i += 1;
    }

    fn get_token(&mut self) -> Token {
        let token = self.peek();
        self.skip();
        return token;
    }

    fn expect_token(&mut self, expected_token: Token) -> Parser<()> {
        let token = self.get_token();
        if token == expected_token {
            Ok(())
        } else {
            Err(ParseError::Expected { expected_token, token })
        }
    }

    fn parse_expr(&mut self) -> Parser<CExpr> {
        match self.get_token() {
            Token::NUMBER(number) => {
                Ok(CExpr::Number(number))
            },
            _ => Err(ParseError::NumberExpected),
        }
    }

    fn parse_type(&mut self) -> Parser<Type> {
        let token = self.get_token();
        if token == Token::KEYWORD("int".to_string()) {
            Ok("int".to_string())
        } else {
            Err(ParseError::TypeExpected)
        }
    }

    fn parse_name(&mut self) -> Parser<Type> {
        let token = self.get_token();
        if let Token::IDENTIFIER(name) = token {
            Ok(name)
        } else {
            Err(ParseError::NameExpected)
        }
    }

    fn parse_stm(&mut self) -> Parser<CStm> {
        if self.peek() == Token::KEYWORD("return".to_string()) {
            self.skip();
            let expr = self.parse_expr()?;
            Ok(CStm::Return(expr))
        } else {
            Err(ParseError::Expected { expected_token: Token::KEYWORD("return".to_string()), token: self.peek() })
        }
    }

    fn parse_block(&mut self) -> Parser<CBlock> {
        self.expect_token(Token::LBRACE)?;
        let mut stms = vec![];
        while self.peek() != Token::RBRACE {
            let stm = self.parse_stm()?;
            stms.push(stm);
            if self.peek() == Token::SEMICOLON {
                self.skip();
            } else {
                self.expect_token(Token::RBRACE)?;
            }
        }
        self.expect_token(Token::RBRACE)?;
        Ok(stms)
    }

    fn parse_decl(&mut self) -> Parser<CDecl> {
        let return_tp = self.parse_type()?;
        let name = self.parse_name()?;
        self.expect_token(Token::LPAREN)?;
        // TODO: allow args
        let parameters = vec![];
        self.expect_token(Token::RPAREN)?;
        let body = self.parse_block()?;

        Ok(CDecl::Fun { return_tp, name, parameters, body })
    }

    fn parse_decls(&mut self) -> Parser<CProgram> {
        let decl = self.parse_decl()?;
        let mut result = vec![decl];
        if self.peek() != Token::EOF {
            self.skip();
            let mut decls = self. parse_decls()?;
            result.append(&mut decls);
        }
        Ok(result)
    }
}

pub fn parse_program(tokens: Vec<Token>) -> Result<CProgram, ParseError> {
    ParseState::new(tokens).parse_decls()
}
