use anyhow::{Result, bail};
use crate::arch::Register;

/// Represents position of a character in code
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodePosition {
  pub char: usize,
  pub row: usize,
  pub col: usize,
}
impl CodePosition {
  /// Create a new [`CodePosition`]
  pub fn new(char: usize, row: usize, col: usize) -> Self {
    Self { char, row, col }
  }
  /// Creates a new [`CodePosition`] pointing to the next character
  pub fn next(&self) -> Self {
    Self {
      char: self.char + 1,
      col: self.col + 1,
      ..*self
    }
  }
  /// Creates a new [`CodePosition`] pointing to the start of the next row
  pub fn next_row(&self) -> Self {
    Self {
      char: self.char + 1,
      col: 0,
      row: self.row + 1,
      ..*self
    }
  }
  /// Calls [`CodePosition::next_row`] if the character is newline, otherwise [`CodePosition::next`]
  pub fn next_auto(&self, chr: char) -> Self {
    match chr {
      '\n' => self.next_row(),
      _ => self.next()
    }
  }
}
impl Default for CodePosition {
  fn default() -> Self {
    Self::new(0, 0, 0)
  }
}

#[derive(Clone, Debug)]
pub enum TokenType {
  Instruction(String),
  StringLiteral(String),
  // CharLiteral(char),
  IntegerLiteral(isize),
  SymbolLiteral(String),
  Symbol(String),
  RegisterPointer(Register),
  Whitespace,
  Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
  pub token: TokenType,
  pub position: CodePosition
}

#[derive(Clone)]
pub struct Tokenizer<'a> {
  code: &'a str,
  tokens: Vec<Token>,
  position: CodePosition
}
impl<'a> Tokenizer<'a> {
  /// Creates a new [`Tokenizer`]
  /// 
  /// Please note that in most cases `Tokenizer::tokenize`  should be used instead!
  pub fn new(code: &'a str) -> Self {
    Self {
      code,
      tokens: Vec::new(),
      position: CodePosition::default()
    }
  }

  /// Shorthand function.
  /// 
  /// This code:
  /// ```no_run
  /// let code = "...";
  /// let tokens = {
  ///   let mut tokenizer = Tokenizer::new(code);
  ///   tokenizer.run()?;
  ///   tokenizer.finish()
  /// };
  /// ```
  /// Is equivalent to this:
  /// ```no_run
  /// let code = "...";
  /// let tokens = Tokenizer::tokenize(code)?;
  /// ```
  pub fn tokenize(code: &'a str) -> Result<Vec<Token>> {
    let mut tokenizer = Self::new(code);
    tokenizer.run()?;
    Ok(tokenizer.finish())
  }

  fn peek(&self, offset: isize) -> Option<char> {
    self.code.chars().nth(self.position.char.wrapping_add_signed(offset))
  }
  fn take(&mut self) -> Option<char> {
    let chr = self.peek(0)?;
    self.position = self.position.next_auto(chr);
    Some(chr)
  }

  /// Compute at most one token
  /// Returns true if EOF
  pub fn step(&mut self) -> Result<bool> {
    macro_rules! err {
      ($message: expr) => {{
        bail!("Error on line {}, column {}\t||\t{}", self.position.row + 1, self.position.col + 1, $message);
      }};
    }

    let start_pos = self.position;
    let chr = match self.peek(0) {
      Some(x) => x,
      None => return Ok(true),
    };


    // WHITESPACE TOKEN


    if chr.is_whitespace() {
      loop {
        match self.peek(0) {
          Some(x) => {
            if x.is_whitespace() {
              self.take().unwrap();
            } else {
              break
            }
          }
          None => break
        }
      }
      self.tokens.push(Token {
        token: TokenType::Whitespace,
        position: start_pos,
      });
      return Ok(false);
    }


    //INTEGER TOKEN


    if chr.is_digit(10) {
      let radix = if chr == '0' {
        match self.peek(1) {
          Some('x') => 16,
          Some('o') => 8,
          Some('b') => 2,
          _ => 10
        }
      } else { 10 };

      if radix != 10 {
        self.take().unwrap();
        self.take().unwrap();
        match self.peek(0) {
          Some(x) => {
            if !x.is_digit(radix) {
              err!("Malformed integer: No integer body")
            }
          }
          None => err!("Malformed integer: EOF before integer body")
        }
      }

      let mut value: isize = 0;
      loop {
        let chr = match self.peek(0) {
          Some(x) => x,
          None => break,
        };
        match chr.to_digit(radix) {
          Some(x) => {
            value *= radix as isize;
            value += x as isize;
            self.take().unwrap();
          }
          None => break
        }
      }

      self.tokens.push(Token {
        token: TokenType::IntegerLiteral(value),
        position: start_pos,
      });
      return Ok(false);
    }


    //STRING TOKEN


    if chr == '"' {
      self.take().unwrap();
      let mut str = String::new();
      loop {
        let char = match self.take() {
          Some('\\') => {
            match self.take() {
              //TODO more escape seq and hex escape
              Some('n') => str.push('\n'),
              Some('r') => str.push('\r'),
              Some('"') => str.push('"'),
              Some(x) => err!(format!("Invalid escape sequence: \\{}", x)),
              None => err!("Malformed escape sequence: EOF reached")
            };
          }
          Some('"') => {
            break
          }
          Some(x) => {
            str.push(x);
          }
          None => err!(format!("Unterminated string (starts on line {}, column {})", start_pos.row + 1, start_pos.col + 1))
        };
      }

      self.tokens.push(Token {
        token: TokenType::StringLiteral(str),
        position: start_pos,
      });
      return Ok(false);
    }


    // INSTR TOKEN

    if chr.is_alphabetic() {
      //TODO
    }

    err!("Invalid token: No token matched");
  }

  /// Run tokenizer until the end of file (EOF)
  pub fn run(&mut self) -> Result<()> {
    while !(self.step()?) {}
    Ok(())
  }
  /// Consumes [`Tokenizer`], returns a vector of [`Token`]s
  /// 
  /// Use `Tokenizer::run` to actually tokenize the code!
  pub fn finish(self) -> Vec<Token> {
    self.tokens
  }
}
