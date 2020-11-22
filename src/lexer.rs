use crate::token::{Token, TokenType};
use log::*;
use onig::Regex;

pub fn lex(file: &String) {
  let file = remove_comments(file);
  let stuff = Regex::new("\\.[a-zA-Z]+").unwrap();
  let matches = stuff.find_iter(&file);
  let tokens: Vec<Token> = matches
    .map(|m| {
      let text: String = file.get(m.0..m.1).unwrap().chars().collect();
      Token::new(&text, TokenType::OParen, m.0, m.1)
    })
    .collect();
  for token in tokens {
    debug!("{}", token);
  }
}

// prepass to remove all comments from the file but leave the rest.
// if this is too slow we can revisit
fn remove_comments(file: &String) -> String {
  let mut deleting = false;
  file
    .chars()
    .filter(|c| {
      match c {
        ';' => deleting = true,
        '\n' => deleting = false,
        _ => (),
      }
      deleting == false
    })
    .collect()
}
