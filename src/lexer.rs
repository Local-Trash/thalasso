use std::str::Chars;

pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>
}

impl<'a, 'b> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars(),
        }
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        loop {
            let token = self.read_next_token();

            if token.kind == Kind::EoF {
                tokens.push(token);
                break;
            }

            tokens.push(token);
        }

        self.check_keywords(&mut tokens);

        tokens
    }

    pub fn print_tokens(&self, tokens: &Vec<Token>) {
        print!("{{ ");
        for token in tokens {
            if token.kind == Kind::String {
                let value = self.get_value(token);
                print!(r#"{:?}: "{}", "#, token.kind, &value[1..value.len() - 1]);
                continue;
            }
            print!(r#"{:?}: "{}", "#, token.kind, self.get_value(token));
        }
        println!("}}");
    }

    fn read_next_kind(&mut self) -> Kind {
        while let Some(c) = self.chars.next() {
            if c.is_ascii_whitespace() {
                continue;
            }
            match c {
                '+' => {
                    if let Some(n) = self.peek() {
                        if n == '=' {
                            self.chars.next();
                            return Kind::PlusEqual;
                        }
                    }
                    return Kind::Plus;
                }
                '-' => {
                    if let Some(n) = self.peek() {
                        if n == '=' {
                            self.chars.next();
                            return Kind::MinusEqual;
                        }
                    }
                    return Kind::Minus;
                }
                '/' => {
                    if let Some(n) = self.peek() {
                        match n {
                            '=' => {
                                self.chars.next();
                                return Kind::DivideEqual;
                            }
                            '*' => {
                                self.chars.next();
                                loop {
                                    // This loops through the next 2 chars to check if they are the end of comment
                                    if let Some(d) = self.peek() {
                                        if d == '*' {
                                            self.chars.next();
                                            if let Some(e) = self.peek() {
                                                if e == '/' {
                                                    self.chars.next();
                                                    return Kind::Comment;
                                                }
                                            }
                                        }
                                    }
                                    self.chars.next();
                                }
                            }
                            _ => {}
                        }
                    }
                    return Kind::Divide;
                }
                '*' => {
                    if let Some(n) = self.peek() {
                        if n == '=' {
                            self.chars.next();
                            return Kind::MultiEqual;
                        }
                    }
                    return Kind::Multi;
                }
                '=' => {
                    if let Some(n) = self.peek() {
                        if n == '=' {
                            self.chars.next();
                            return Kind::EqualEqual;
                        }
                    }
                    return Kind::Equal;
                }
                ';' => {
                    return Kind::SemiColon;
                }
                '(' => {
                    return Kind::StartParen;
                }
                ')' => {
                    return Kind::EndParen;
                }
                '{' => {
                    return Kind::StartBrak;
                }
                '}' => {
                    return Kind::EndBrak;
                }
                ':' => {
                    return Kind::Colon;
                }
                '\"' => {
                    while let Some(t) = self.peek() {
                        if t == '\"' {
                            self.chars.next();
                            return Kind::String;
                        }
                        self.chars.next();
                    }
                }
                c if c.is_numeric() => {
                    while let Some(n) = self.peek() {
                        if !n.is_numeric() && n != '.' {
                            break;
                        }
                        self.chars.next();
                    }
                    return Kind::Numeric;
                }
                c if c.is_alphabetic() => {
                    while let Some(s) = self.peek() {
                        if !s.is_alphanumeric() {
                            break;
                        } 
                        self.chars.next();
                    }
                    return Kind::Ident;
                }
                _ => {
                    
                }
            }
        }
        Kind::EoF
    }

    fn read_next_token(&mut self) -> Token {
        self.remove_whitespace();
        let start = self.offset();
        let kind = self.read_next_kind();
        let end = self.offset();
        Token {kind, start, end}
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn remove_whitespace(&mut self) {
        while let Some(n) = self.peek() {
            if !n.is_ascii_whitespace() {
                break;
            }
            self.chars.next();
        }
    }

    fn get_value(&'b self, token: &Token) -> &'b str {
        let value = &self.source[token.start..token.end];

        return value;
    }

    fn check_keywords(&'b self, tokens: &mut Vec<Token>) {
        for token in tokens {
            if token.kind == Kind::Ident {
                match self.get_value(&token) {
                    "func" => {
                        token.kind = Kind::Func;
                    }
                    "let" => {
                        token.kind = Kind::Let;
                    }
                    "mut" => {
                        token.kind = Kind::Mut;
                    }
                    "struct" => {
                        token.kind = Kind::Struct;
                    }
                    "return" => {
                        token.kind = Kind::Return;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    EoF,
    Comment,
    SemiColon,
    Colon,

    StartParen,
    EndParen,
    StartBrak,
    EndBrak,

    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Divide,
    DivideEqual,
    Multi,
    MultiEqual,
    Equal,
    EqualEqual,

    Numeric,
    Ident,
    String,

    Func,
    Let,
    Mut,
    Struct,
    Return,

}