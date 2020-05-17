pub mod token {

    #[derive(Debug, PartialEq)]
    pub struct Token {
        pub kind: TokenType,
        pub literal: String,
    }

    #[derive(Debug, PartialEq)]
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
        FUNCTION,
        LET,
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

        pub fn read_char(&mut self) {
            if self.read_pos >= self.input.len() {
                self.ch = '\0';
            } else {
                self.ch = self.input.chars().nth(self.read_pos).unwrap();
            }
            self.pos = self.read_pos;
            self.read_pos += 1;
        }

        pub fn next_token(&mut self) -> Token {
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
                _ => Token {
                    kind: TokenType::EOF,
                    literal: "".to_string(),
                },
            };

            self.read_char();
            tok
        }
    }
}
