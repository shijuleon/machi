use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::fmt;

#[derive(Debug)]
enum TokenType {
  // Single-character tokens.
  LEFT_PAREN,
  RIGHT_PAREN,
  LEFT_BRACE,
  RIGHT_BRACE,
  COMMA,
  DOT,
  MINUS,
  PLUS,
  SEMICOLON,
  SLASH,
  STAR,

  // One or two character tokens.
  BANG,
  BANG_EQUAL,
  EQUAL,
  EQUAL_EQUAL,
  GREATER,
  GREATER_EQUAL,
  LESS,
  LESS_EQUAL,

  // Literals.
  IDENTIFIER,
  STRING,
  NUMBER,

  // Keywords.
  MACHI,
  SOLLU,
  SARI,
  IPO,
  ILANA,
  PANNU,
  PODHUM,

  EOF,
}

struct Token {
  token_type: TokenType,
  lexeme: String,
  line: u32,
}

impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.token_type)
  }
}

struct Scanner {
  source: String,
  lexemes: Vec<String>,
  current: u32,
  tokens: Vec<Token>,
}

enum MachiError {
  UnexpectedIdentifier,
}

impl Scanner {
  fn new(source: &str) -> Self {
    Scanner {
      source: source.to_string(),
      current: 0,
      tokens: Vec::new(),
      lexemes: Vec::new(),
    }
  }

  // fn scan_tokens(&mut self) {
  //   let mut lexemes: Vec<&str> = Vec::new();

  //   for lexeme in lexemes {
  //      self.scan_token(&lexeme);
  //   }
  // }

  fn scan_tokens(&mut self) -> Result<(), MachiError> {
    let source = self.source.clone();

    for lexeme in source.split(" ") {
      let lexeme = lexeme.trim();
      self.lexemes.push(lexeme.to_string());
      
      match lexeme {
        "machi" => self.add_token(lexeme, TokenType::MACHI),
        "sollu" => self.add_token(lexeme, TokenType::SOLLU),
        "sari" => self.add_token(lexeme, TokenType::SARI),
        "ipo" => self.add_token(lexeme, TokenType::IPO),
        "pannu" => self.add_token(lexeme, TokenType::PANNU),
        "podhum" => self.add_token(lexeme, TokenType::PODHUM),
        "ilana" => self.add_token(lexeme, TokenType::ILANA),
        "==" => self.add_token(lexeme, TokenType::EQUAL_EQUAL),
        "<=" => self.add_token(lexeme, TokenType::LESS_EQUAL),
        ">=" => self.add_token(lexeme, TokenType::GREATER_EQUAL),
        "-" => self.add_token(lexeme, TokenType::MINUS),
        "+" => self.add_token(lexeme, TokenType::PLUS),
        "*" => self.add_token(lexeme, TokenType::STAR),
        "=" => self.add_token(lexeme, TokenType::EQUAL),
        "<" => self.add_token(lexeme, TokenType::LESS),
        ">" => self.add_token(lexeme, TokenType::GREATER),
        _ => {
          if lexeme.starts_with("\"") {
            self.add_token(lexeme, TokenType::STRING)
          } else {
            match lexeme.parse::<u64>() {
              Ok(_) => self.add_token(lexeme, TokenType::NUMBER),
              Err(e) => {
                println!("Unexpected identifier: {}: {}", lexeme, e);
                return Err(MachiError::UnexpectedIdentifier);
              }
            }
          }
        }
      }
    }

    Ok(())
  }

  fn get_next_lexeme(&mut self) -> &str {
    return self.lexemes[self.current as usize + 1].as_str();
  }

  fn is_at_end(self) {}

  fn add_token(&mut self, lexeme: &str, token_type: TokenType) {
    let token = Token {
      lexeme: lexeme.to_string(),
      token_type: token_type,
      line: 0,
    };

    self.tokens.push(token);
  }
}

fn run_prompt() {
  let mut input_line = String::new();

  loop {
    print!("machi> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_line).unwrap();
    if input_line.trim() == "exit()" {
      break;
    }

    let mut scanner = Scanner::new(&input_line);
    scanner.scan_tokens();
    println!("{:?}", scanner.lexemes);

    for token in scanner.tokens {
      print!("{:?} ", token);
    }
    println!();

    input_line.clear();
  }
}

fn run_script(filename: &str) {
  let mut f = File::open(filename).unwrap();
  let mut script: String = String::new();
  f.read_to_string(&mut script).unwrap();
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.len() {
    1 => run_prompt(),
    2 => run_script(&args[1]),
    _ => println!("Usage: machi <script>"),
  }
}
