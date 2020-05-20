pub mod token {

    #[derive(Debug, PartialEq)]
    pub struct Token {
        pub kind: TokenType,
        pub literal: String,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenType {
        ILLEGAL,
        EOF,
        IDENT,     // add, foobar, x, y
        INT,       // 12355
        ASSIGN,    // =
        PLUS,      // +
        COMMA,     // ,
        SEMICOLON, // ;
        LPAREN,    // (
        RPAREN,    // )
        LBRACE,    // {
        RBRACE,    // }
        FUNCTION,  // fn
        LET,       // let
    }

    // Since it's really hard to initialize a static hashmap without
    // the use of a crate, this is good stopgap
    pub fn keyword(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            _ => TokenType::IDENT,
        }
    }
}

pub mod lexer {
    use super::token::*;
    pub struct Lexer {
        pub input: String,
        pub pos: usize,      // current position in input (points to current char)
        pub read_pos: usize, // current read post (after current char)
        pub ch: char,        // current char under examination
    }

    impl Lexer {
        pub fn new(input: String) -> Lexer {
            assert!(input.len() > 0, "Source code should not be empty");
            let mut l = Lexer {
                input,
                pos: 0,
                read_pos: 0,
                ch: '\0',
            };
            l.read_char();
            l
        }

        // Reads a single character from the input stream
        // by advancing the position and read position
        pub fn read_char(&mut self) {
            if self.read_pos >= self.input.len() {
                self.ch = '\0';
            } else {
                self.ch = self.input.chars().nth(self.read_pos).unwrap();
            }
            self.pos = self.read_pos;
            self.read_pos += 1;
        }

        // If the character is detected as being alphabetic,
        // all subsequent alhpabetic characters are collected
        // since this would be an identifier
        pub fn read_ident(&mut self) -> String {
            let start_pos = self.pos;

            // Allows for names like foo_bar
            while self.ch.is_alphabetic() || self.ch == '_' {
                self.read_char();
            }

            // return the ident range
            self.input.get(start_pos..self.pos).unwrap().to_string()
        }

        pub fn read_digit(&mut self) -> String {
            let start_pos = self.pos;

            while self.ch.is_digit(10) {
                self.read_char();
            }

            // return the ident range
            self.input.get(start_pos..self.pos).unwrap().to_string()
        }

        // Consumes all unicode whitespaces
        pub fn skip_whtspc(&mut self) {
            while self.ch.is_whitespace() {
                self.read_char()
            }
        }

        pub fn next_token(&mut self) -> Token {
            self.skip_whtspc();
            let tok: Token = match self.ch {
                '=' => Token {
                    kind: TokenType::ASSIGN,
                    literal: "=".to_string(),
                },
                ';' => Token {
                    kind: TokenType::SEMICOLON,
                    literal: ";".to_string(),
                },
                '(' => Token {
                    kind: TokenType::LPAREN,
                    literal: "(".to_string(),
                },
                ')' => Token {
                    kind: TokenType::RPAREN,
                    literal: ")".to_string(),
                },
                ',' => Token {
                    kind: TokenType::COMMA,
                    literal: ",".to_string(),
                },
                '+' => Token {
                    kind: TokenType::PLUS,
                    literal: "+".to_string(),
                },
                '{' => Token {
                    kind: TokenType::LBRACE,
                    literal: "{".to_string(),
                },
                '}' => Token {
                    kind: TokenType::RBRACE,
                    literal: "}".to_string(),
                },
                '\0' => Token {
                    kind: TokenType::EOF,
                    literal: "".to_string(),
                },
                c => {
                    if c.is_alphabetic() {
                        let literal = self.read_ident();
                        let kind = keyword(literal.as_str());
                        return Token { kind, literal };
                    } else if c.is_digit(10) {
                        let literal = self.read_digit();
                        let kind = TokenType::INT;
                        return Token { kind, literal };
                    } else {
                        Token {
                            kind: TokenType::ILLEGAL,
                            literal: c.to_string(),
                        }
                    }
                }
            };

            self.read_char();
            tok
        }
    }
}
