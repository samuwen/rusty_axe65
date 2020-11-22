use crate::token::{Token, TokenType};
use log::*;
use onig::{FindMatches, Regex};

pub fn lex(file: &String) -> Vec<Token> {
  let file = remove_comments(file);
  let mut out_vec = Vec::with_capacity(file.len());
  out_vec.append(&mut get_tokens_of_type(
    &file,
    "(?<![a-zA-Z0-9])\\.[a-zA-Z]+",
    &TokenType::Directive,
  ));
  out_vec.append(&mut get_tokens_of_type(
    &file,
    "(?<!\\$|#|%)[a-zA-Z\\.][a-zA-Z0-9_/:]+",
    &TokenType::Identifier,
  ));
  out_vec.sort();
  out_vec
}

fn get_tokens_of_type(file: &String, regex: &str, t_type: &TokenType) -> Vec<Token> {
  trace!("Finding matches for {}", regex);
  let re = Regex::new(regex).unwrap();
  let matches = re.find_iter(&file);
  match t_type {
    TokenType::Identifier => filter_for_identifier(file, matches),
    _ => find_matches(file, matches, t_type),
  }
}

fn find_matches(file: &String, matches: FindMatches, t_type: &TokenType) -> Vec<Token> {
  matches
    .map(|m| get_token_from_text(file, m, t_type))
    .collect()
}

fn filter_for_identifier(file: &String, matches: FindMatches) -> Vec<Token> {
  matches
    .filter(|m| {
      let text = get_text_from_file_at_loc(file, m);
      let found = text.chars().any(|c| c == '.' || c == ':');
      !found
    })
    .map(|m| get_token_from_text(file, m, &TokenType::Identifier))
    .collect()
}

fn get_token_from_text(f: &String, m: (usize, usize), t: &TokenType) -> Token {
  let text = get_text_from_file_at_loc(f, &m);
  Token::new(String::from(text.trim()), t, m.0, m.1)
}

fn get_text_from_file_at_loc(f: &String, m: &(usize, usize)) -> String {
  f.get(m.0..m.1).unwrap().chars().collect()
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
