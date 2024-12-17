#[derive(Debug)]
pub enum LexingError {
    UnexpectedCharacter { line: i32 },
    UnterminatedString { line: i32 },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Equal,

    // Literals
    Function { name: String },
    Label { name: String },
    Register { name: String },
    String { content: String },
    IntLiteral { value: i64 },

    // Types
    I32,

    // Keywords
    Add,
    Sub,
    Mul,
    Div,
    Define,
    Return,
    Call,
    Exit,
    Jmp,
    Branch,
    ICmp,

    // Compare Types
    LE,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: i32,
}

#[derive(Default)]
pub struct Scanner {
    index: usize,
    line: i32,
    chars: Vec<char>,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        let chars: Vec<char> = source.chars().collect();
        Scanner {
            line: 0,
            index: 0,
            chars,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LexingError> {
        let mut tokens = Vec::new();

        self.line = 0;

        while self.scan_token(&mut tokens)? {}

        tokens.push(Token {
            token_type: TokenType::EOF,
            line: self.line,
        });

        self.index = 0;
        self.line = 0;

        Ok(tokens)
    }

    pub fn scan_token(&mut self, tokens: &mut Vec<Token>) -> Result<bool, LexingError> {
        let c = self.peek();
        if c.is_none() {
            return Ok(false);
        }
        let c = c.unwrap();

        match c {
            '(' => self.make_token(tokens, TokenType::LeftParen),
            ')' => self.make_token(tokens, TokenType::RightParen),
            '{' => self.make_token(tokens, TokenType::LeftBrace),
            '}' => self.make_token(tokens, TokenType::RightBrace),
            ',' => self.make_token(tokens, TokenType::Comma),
            ':' => self.make_token(tokens, TokenType::Colon),
            '=' => self.make_token(tokens, TokenType::Equal),

            '"' => {
                let start_line = self.line;
                let mut string = Vec::new();

                self.consume();

                while self.peek().is_some_and(|x| x != '"') {
                    let s = self.consume();

                    if s == '\n' {
                        self.line += 1;
                    }

                    string.push(s);
                }

                if self.peek().is_none() {
                    return Err(LexingError::UnterminatedString { line: start_line });
                }

                // Strip the final "
                self.consume();

                tokens.push(Token {
                    line: self.line,
                    token_type: TokenType::String {
                        content: string.iter().collect(),
                    },
                });
            }

            '%' => {
                let mut string = vec![self.consume()];
                self.scan_identifer(&mut string);

                tokens.push(Token {
                    line: self.line,
                    token_type: TokenType::Register {
                        name: string.iter().collect(),
                    },
                });
            }

            '@' => {
                self.consume();

                let mut string = vec![];
                self.scan_identifer(&mut string);

                let function_name: String = string.iter().collect();

                tokens.push(Token {
                    line: self.line,
                    token_type: TokenType::Function {
                        name: function_name.clone(),
                    },
                });

                tokens.push(Token {
                    token_type: TokenType::Label {
                        name: function_name,
                    },
                    line: self.line,
                });
                tokens.push(Token {
                    token_type: TokenType::Colon,
                    line: self.line,
                });
            }

            ' ' | '\r' | '\t' => {
                self.consume();
            }

            '\n' => {
                self.line += 1;
                self.consume();
            }

            _ => {
                if c.is_ascii_digit() {
                    let mut num = Vec::new();

                    while self.peek().is_some_and(|x| x.is_ascii_digit()) {
                        num.push(self.consume());
                    }

                    tokens.push(Token {
                        line: self.line,
                        token_type: TokenType::IntLiteral {
                            value: (num.iter().collect::<String>()).parse().unwrap(),
                        },
                    });
                } else if c.is_ascii_alphabetic() {
                    let mut word = vec![self.consume()];
                    self.scan_identifer(&mut word);

                    match word.iter().collect::<String>().as_str() {
                        // Types
                        "i32" => self.make_token(tokens, TokenType::I32),

                        // Keywords
                        "add" => self.make_token(tokens, TokenType::Add),
                        "sub" => self.make_token(tokens, TokenType::Sub),
                        "mul" => self.make_token(tokens, TokenType::Mul),
                        "div" => self.make_token(tokens, TokenType::Div),
                        "exit" => self.make_token(tokens, TokenType::Exit),
                        "define" => self.make_token(tokens, TokenType::Define),
                        "ret" => self.make_token(tokens, TokenType::Return),
                        "call" => self.make_token(tokens, TokenType::Call),
                        "jmp" => self.make_token(tokens, TokenType::Jmp),
                        "cmp" => self.make_token(tokens, TokenType::ICmp),
                        "branch" => self.make_token(tokens, TokenType::Branch),

                        // Cmp Types
                        "le" => self.make_token(tokens, TokenType::LE),

                        _ => {
                            tokens.push(Token {
                                token_type: TokenType::Label {
                                    name: word.into_iter().collect(),
                                },
                                line: self.line,
                            });
                        }
                    }
                } else {
                    return Err(LexingError::UnexpectedCharacter { line: self.line });
                }
            }
        }

        Ok(true)
    }

    fn make_token(&mut self, tokens: &mut Vec<Token>, token_type: TokenType) {
        self.consume();

        tokens.push(Token {
            token_type,
            line: self.line,
        });
    }

    fn scan_identifer(&mut self, word: &mut Vec<char>) {
        while self.peek().is_some_and(|x| x.is_alphanumeric()) {
            word.push(self.consume());
        }
    }

    fn peek(&self) -> Option<char> {
        let c = self.chars.get(self.index);

        match c {
            None => None,
            Some(char) => Some(char.clone()),
        }
    }

    fn consume(&mut self) -> char {
        self.index += 1;
        self.chars.get(self.index - 1).unwrap().clone()
    }
}
