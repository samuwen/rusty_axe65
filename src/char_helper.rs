pub fn is_id_start(c: char) -> bool {
  c.is_ascii_alphabetic() || c == '_'
}

pub fn is_hex_signifier(c: char) -> bool {
  c == '$'
}

pub fn is_bin_signifier(c: char) -> bool {
  c == '%'
}

pub fn is_dec_signifier(c: char) -> bool {
  c.is_digit(10)
}

pub fn is_num_signifier(c: char) -> bool {
  is_hex_signifier(c) || is_bin_signifier(c) || is_dec_signifier(c)
}

pub fn is_ctrl_command_signifier(c: char) -> bool {
  c == '.'
}

pub fn is_local_label_signifier(c: char) -> bool {
  c == '@'
}

pub fn is_hex_number(c: char) -> bool {
  c.is_digit(16)
}

pub fn is_bin_number(c: char) -> bool {
  c.is_digit(2)
}

pub fn is_dec_number(c: char) -> bool {
  c.is_digit(10)
}

pub fn is_identifier(c: char) -> bool {
  c.is_ascii_alphabetic() || c.is_digit(10) || c == '_'
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_hex_num() {
    assert!(is_hex_number('B'));
    assert!(is_hex_number('b'));
    assert!(is_hex_number('a'));
    assert!(is_hex_number('f'));
    assert!(is_hex_number('3'));
    assert!(is_hex_number('7'));
  }
}
