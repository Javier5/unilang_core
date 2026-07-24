#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    Operator(String),
    Symbol(String),
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::CharIndices<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.char_indices();
        let current_char = chars.next().map(|(_, c)| c);
        Self { input, chars, current_char }
    }

    fn advance(&mut self) {
        self.current_char = self.chars.next().map(|(_, c)| c);
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
                continue;
            }
            if c.is_alphabetic() {
                let start = self.chars.offset();
                // Lógica simplificada de extracción de identificadores
                self.advance();
                return Token::Identifier(c.to_string());
            }
            self.advance();
        }
        Token::EOF
    }
}
