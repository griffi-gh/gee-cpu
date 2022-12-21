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
    self.code.chars().nth(self.position.char)
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
        bail!("Error on line {}:{}\n\t- {}", self.position.row, self.position.col, $message);
      }};
    }
    let chr = match self.peek(0) {
      None => return Ok(true),
      Some(x) => x
    };
    if chr.is_whitespace() {
      self.tokens.push(Token {
        token: TokenType::Whitespace,
        position: self.position,
      });
      return Ok(false);
    }
    if chr.is_digit(10) {
      let chr = self.take().unwrap();

      let mut radix = if chr == '0' {
        match self.peek(0) {
          Some('x') => 16,
          Some('o') => 8,
          Some('b') => 2,
          _ => 10
        }
      } else { 10 };

      if radix != 10 {
        self.take().unwrap();
        if self.peek(0).is_none() {
          err!("Malformed integer: EOF before integer body");
        }
      }

      let mut value = 0;
      loop {
        let chr = self.peek(0);
        if chr.is_none() {
          break
        }
        let chr = chr.unwrap();
        match chr.to_digit(radix) {
          Some(x) => {
            self.take().unwrap();
            value *= radix;
            value += x;
          }
          None => break
        }
      }
    }
    match chr {

      _ => ()
    }
    Ok(false)
  }
  /// Run tokenizer until the end of file (EOF)
  pub fn run(&mut self) -> Result<()> {
    while !self.step()? {}
    Ok(())
  }
  /// Consumes [`Tokenizer`], returns a vector of [`Token`]s
  /// 
  /// Use `Tokenizer::run` to actually tokenize the code!
  pub fn finish(self) -> Vec<Token> {
    self.tokens
  }
}
