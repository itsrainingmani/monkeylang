use monkey::{lexer::Lexer, token, token::TokenType};

#[test]
fn test_next_token() {
    let mut lex = Lexer::new(String::from("=+(){},;"));

    let expected_tokens = vec![
        token::Token {
            kind: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        token::Token {
            kind: TokenType::PLUS,
            literal: "+".to_string(),
        },
        token::Token {
            kind: TokenType::LPAREN,
            literal: "(".to_string(),
        },
        token::Token {
            kind: TokenType::RPAREN,
            literal: ")".to_string(),
        },
        token::Token {
            kind: TokenType::LBRACE,
            literal: "{".to_string(),
        },
        token::Token {
            kind: TokenType::RBRACE,
            literal: "}".to_string(),
        },
        token::Token {
            kind: TokenType::COMMA,
            literal: ",".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        // token::Token {
        //     kind: token::EOF.to_string(),
        //     literal: "".to_string(),
        // },
    ];

    let mut actual_tokens: Vec<token::Token> = Vec::new();

    for _ in expected_tokens.iter() {
        let tok: token::Token = lex.next_token();
        actual_tokens.push(tok);
    }

    assert_eq!(expected_tokens, actual_tokens);
}

#[test]
fn test_whitespace_consumption() {
    let mut lex = Lexer::new(String::from("hello  \n\r\t  world"));

    let whtspc: Vec<bool> = lex.input.chars().map(|x| x.is_whitespace()).collect();

    println!("{:#?}", whtspc);
}

#[test]
fn test_lexer_internal_state_change() {
    let mut lex = Lexer::new(String::from("hello  \n\r\t  world"));

    let initial_lex_pos = (lex.pos, lex.read_pos);

    for _ in 0..=2 {
        lex.next_token();
    }

    println!("{:#?}", initial_lex_pos);
    println!("{:#?}", (lex.pos, lex.read_pos));
    assert_ne!(initial_lex_pos, (lex.pos, lex.read_pos));
}

#[test]
fn test_read_minimal_source() {
    let mut lex = Lexer::new(String::from("let five = 5;\nlet ten = 10;"));

    let expected_tokens = vec![
        token::Token {
            kind: TokenType::LET,
            literal: "let".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "five".to_string(),
        },
        token::Token {
            kind: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        token::Token {
            kind: TokenType::INT,
            literal: "5".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        token::Token {
            kind: TokenType::LET,
            literal: "let".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "ten".to_string(),
        },
        token::Token {
            kind: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        token::Token {
            kind: TokenType::INT,
            literal: "10".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
    ];

    let mut actual_tokens: Vec<token::Token> = Vec::new();

    for _ in expected_tokens.iter() {
        let tok: token::Token = lex.next_token();
        actual_tokens.push(tok);
    }

    assert_eq!(expected_tokens, actual_tokens);
}

#[test]
fn test_read_expanded_source() {
    let mut lex = Lexer::new(String::from(
        "let five = 5;\nlet ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        ",
    ));

    let expected_tokens = vec![
        token::Token {
            kind: TokenType::LET,
            literal: "let".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "five".to_string(),
        },
        token::Token {
            kind: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        token::Token {
            kind: TokenType::INT,
            literal: "5".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        token::Token {
            kind: TokenType::LET,
            literal: "let".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "ten".to_string(),
        },
        token::Token {
            kind: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        token::Token {
            kind: TokenType::INT,
            literal: "10".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        token::Token {
            kind: TokenType::LET,
            literal: "let".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "add".to_string(),
        },
        token::Token {
            kind: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        token::Token {
            kind: TokenType::FUNCTION,
            literal: "fn".to_string(),
        },
        token::Token {
            kind: TokenType::LPAREN,
            literal: "(".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "x".to_string(),
        },
        token::Token {
            kind: TokenType::COMMA,
            literal: ",".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "y".to_string(),
        },
        token::Token {
            kind: TokenType::RPAREN,
            literal: ")".to_string(),
        },
        token::Token {
            kind: TokenType::LBRACE,
            literal: "{".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "x".to_string(),
        },
        token::Token {
            kind: TokenType::PLUS,
            literal: "+".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "y".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        token::Token {
            kind: TokenType::RBRACE,
            literal: "}".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        token::Token {
            kind: TokenType::LET,
            literal: "let".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "result".to_string(),
        },
        token::Token {
            kind: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "add".to_string(),
        },
        token::Token {
            kind: TokenType::LPAREN,
            literal: "(".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "five".to_string(),
        },
        token::Token {
            kind: TokenType::COMMA,
            literal: ",".to_string(),
        },
        token::Token {
            kind: TokenType::IDENT,
            literal: "ten".to_string(),
        },
        token::Token {
            kind: TokenType::RPAREN,
            literal: ")".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        token::Token {
            kind: TokenType::BANG,
            literal: "!".to_string(),
        },
        token::Token {
            kind: TokenType::MINUS,
            literal: "-".to_string(),
        },
        token::Token {
            kind: TokenType::SLASH,
            literal: "/".to_string(),
        },
        token::Token {
            kind: TokenType::ASTERISK,
            literal: "*".to_string(),
        },
        token::Token {
            kind: TokenType::INT,
            literal: "5".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        token::Token {
            kind: TokenType::INT,
            literal: "5".to_string(),
        },
        token::Token {
            kind: TokenType::LT,
            literal: "<".to_string(),
        },
        token::Token {
            kind: TokenType::INT,
            literal: "10".to_string(),
        },
        token::Token {
            kind: TokenType::GT,
            literal: ">".to_string(),
        },
        token::Token {
            kind: TokenType::INT,
            literal: "5".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        token::Token {
            kind: TokenType::IF,
            literal: "if".to_string(),
        },
        token::Token {
            kind: TokenType::LPAREN,
            literal: "(".to_string(),
        },
        token::Token {
            kind: TokenType::INT,
            literal: "5".to_string(),
        },
        token::Token {
            kind: TokenType::LT,
            literal: "<".to_string(),
        },
        token::Token {
            kind: TokenType::INT,
            literal: "10".to_string(),
        },
        token::Token {
            kind: TokenType::RPAREN,
            literal: ")".to_string(),
        },
        token::Token {
            kind: TokenType::LBRACE,
            literal: "{".to_string(),
        },
        token::Token {
            kind: TokenType::RETURN,
            literal: "return".to_string(),
        },
        token::Token {
            kind: TokenType::TRUE,
            literal: "true".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        token::Token {
            kind: TokenType::RBRACE,
            literal: "}".to_string(),
        },
        token::Token {
            kind: TokenType::ELSE,
            literal: "else".to_string(),
        },
        token::Token {
            kind: TokenType::LBRACE,
            literal: "{".to_string(),
        },
        token::Token {
            kind: TokenType::RETURN,
            literal: "return".to_string(),
        },
        token::Token {
            kind: TokenType::FALSE,
            literal: "false".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        token::Token {
            kind: TokenType::RBRACE,
            literal: "}".to_string(),
        },
    ];

    let mut actual_tokens: Vec<token::Token> = Vec::new();

    for _ in expected_tokens.iter() {
        let tok: token::Token = lex.next_token();
        actual_tokens.push(tok);
    }

    assert_eq!(expected_tokens, actual_tokens);
}
