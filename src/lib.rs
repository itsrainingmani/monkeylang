pub mod token {
    pub type TokenType = String;

    #[derive(Debug, PartialEq)]
    pub struct Token {
        pub kind: TokenType,
        pub literal: String,
    }

    pub const ILLEGAL: &str = "ILLEGAL";
    pub const EOF: &str = "EOF";

    // Identifiers + literals
    pub const IDENT: &str = "IDENT";
    pub const INT: &str = "INT";

    pub const ASSIGN: &str = "=";
    pub const PLUS: &str = "+";

    pub const COMMA: &str = ",";
    pub const SEMICOLON: &str = ";";
    pub const LPAREN: &str = "(";
    pub const RPAREN: &str = ")";
    pub const LBRACE: &str = "{";
    pub const RBRACE: &str = "}";

    pub const FUNCTION: &str = "FUNCTION";
    pub const LET: &str = "LET";
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
                    kind: ASSIGN.to_string(),
                    literal: "=".to_string(),
                },
                ';' => Token {
                    kind: SEMICOLON.to_string(),
                    literal: ";".to_string(),
                },
                '(' => Token {
                    kind: LPAREN.to_string(),
                    literal: "(".to_string(),
                },
                ')' => Token {
                    kind: RPAREN.to_string(),
                    literal: ")".to_string(),
                },
                ',' => Token {
                    kind: COMMA.to_string(),
                    literal: ",".to_string(),
                },
                '+' => Token {
                    kind: PLUS.to_string(),
                    literal: "+".to_string(),
                },
                '{' => Token {
                    kind: LBRACE.to_string(),
                    literal: "{".to_string(),
                },
                '}' => Token {
                    kind: RBRACE.to_string(),
                    literal: "}".to_string(),
                },
                '\0' => Token {
                    kind: EOF.to_string(),
                    literal: "".to_string(),
                },
                _ => Token {
                    kind: EOF.to_string(),
                    literal: "".to_string(),
                },
            };

            self.read_char();
            tok
        }
    }
}
