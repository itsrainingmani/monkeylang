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
        COMMA,     // ,
        SEMICOLON, // ;
        LPAREN,    // (
        RPAREN,    // )
        LBRACE,    // {
        RBRACE,    // }

        // Operators
        ASSIGN,   // =
        PLUS,     // +
        MINUS,    // -
        BANG,     // !
        ASTERISK, // *
        SLASH,    // /
        LT,       // <
        GT,       // >
        EQ,       // ==
        NEQ,      // !=

        // Keywords
        FUNCTION, // fn
        LET,      // let
        TRUE,     // true
        FALSE,    // false
        IF,       // if
        ELSE,     // else
        RETURN,   // return
    }

    // Since it's really hard to initialize a static hashmap without
    // the use of a crate, this is good stopgap
    pub fn keyword(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
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
        // all subsequent alphabetic characters are collected
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

        // If the character is detected as being numeric,
        // all subsequent numeric characters are collected
        // since this would be an identifier
        pub fn read_digit(&mut self) -> String {
            let start_pos = self.pos;

            while self.ch.is_digit(10) {
                self.read_char();
            }

            // return the ident range
            self.input.get(start_pos..self.pos).unwrap().to_string()
        }

        // Peeks ahead in the input and returns that char
        // if the read_pos is greater than or equal to the input
        // return the EOF character
        pub fn peek_char(&self) -> char {
            if self.read_pos >= self.input.len() {
                '\0'
            } else {
                self.input.chars().nth(self.read_pos).unwrap()
            }
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
                '=' => {
                    if self.peek_char() == '=' {
                        self.read_char();
                        Token {
                            kind: TokenType::EQ,
                            literal: "==".to_string(),
                        }
                    } else {
                        Token {
                            kind: TokenType::ASSIGN,
                            literal: "=".to_string(),
                        }
                    }
                }
                '+' => Token {
                    kind: TokenType::PLUS,
                    literal: "+".to_string(),
                },
                '-' => Token {
                    kind: TokenType::MINUS,
                    literal: "-".to_string(),
                },
                '!' => {
                    if self.peek_char() == '=' {
                        self.read_char();
                        Token {
                            kind: TokenType::NEQ,
                            literal: "!=".to_string(),
                        }
                    } else {
                        Token {
                            kind: TokenType::BANG,
                            literal: "!".to_string(),
                        }
                    }
                }
                '/' => Token {
                    kind: TokenType::SLASH,
                    literal: "/".to_string(),
                },
                '*' => Token {
                    kind: TokenType::ASTERISK,
                    literal: "*".to_string(),
                },
                '<' => Token {
                    kind: TokenType::LT,
                    literal: "<".to_string(),
                },
                '>' => Token {
                    kind: TokenType::GT,
                    literal: ">".to_string(),
                },
                ';' => Token {
                    kind: TokenType::SEMICOLON,
                    literal: ";".to_string(),
                },
                ',' => Token {
                    kind: TokenType::COMMA,
                    literal: ",".to_string(),
                },
                '(' => Token {
                    kind: TokenType::LPAREN,
                    literal: "(".to_string(),
                },
                ')' => Token {
                    kind: TokenType::RPAREN,
                    literal: ")".to_string(),
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

pub mod repl {
    use super::lexer::*;
    use super::token::*;
    use std::error::Error;
    use std::io::{self, Write};
    pub struct Repl {
        pub prompt: String,
    }

    impl Repl {
        pub fn new(prompt: String) -> Repl {
            Repl { prompt }
        }

        pub fn run(&self) -> Result<(), Box<dyn Error>> {
            println!("Monkey Lang v0.1 - REPL");

            loop {
                let mut input = String::new();
                let mut lexer = Lexer::new(String::from("something"));
                print!("{} ", self.prompt);
                io::stdout().flush()?;
                io::stdin().read_line(&mut input)?;

                lexer.input = input.clone();
                loop {
                    let tok = lexer.next_token();
                    match tok.kind {
                        TokenType::EOF => break,
                        TokenType::IDENT | TokenType::INT => {
                            println!("{:#?}({:#?})", tok.kind, tok.literal);
                        }
                        _ => {
                            println!("{:#?}", tok.kind);
                        }
                    }
                    io::stdout().flush()?;
                }
            }
            Ok(())
        }
    }
}

pub mod ast {
    use super::token::*;
    pub trait Node {
        fn token_literal(&self) -> String;
    }

    pub trait Statement: Node {}

    pub trait Expression: Node {}

    pub struct LetStmt {
        token: Token,          // the Token::LET token
        name: Identifier,      // holds the identifier of the binding
        value: dyn Expression, // the expression that produces a value
    }

    impl Node for LetStmt {
        fn token_literal(&self) -> String {
            self.token.literal.clone()
        }
    }

    impl Statement for LetStmt {}

    pub struct Identifier {
        token: Token,
        value: String,
    }

    impl Node for Identifier {
        fn token_literal(&self) -> String {
            self.token.literal.clone()
        }
    }

    impl Expression for Identifier {}

    // Program is going to be the root node of every AST the Parser produces
    pub struct Program {
        statements: Vec<Box<dyn Statement>>,
    }

    impl Node for Program {
        fn token_literal(&self) -> String {
            if self.statements.len() > 0 {
                self.statements.get(0).unwrap().token_literal()
            } else {
                "".to_string()
            }
        }
    }
}
