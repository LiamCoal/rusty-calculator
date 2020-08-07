extern crate regex;

use self::regex::Regex;

fn squash_whitespace(input: &str) -> String {
    let mut ret: String = String::from(input);
    let regex1 = Regex::new(r"^\s+").unwrap();
    let regex2 = Regex::new(r"\s+$").unwrap();
    let regex3 = Regex::new(r"\s+").unwrap();
    ret = (&*regex1.replace(&*ret, "")).parse().unwrap();
    ret = (&*regex2.replace(&*ret, "")).parse().unwrap();
    ret = (&*regex3.replace_all(&*ret, " ")).parse().unwrap();
    ret
}

pub(crate) fn tokenize(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let squashed = &*squash_whitespace(input);
    squashed.split(" ").for_each(|token| {
        tokens.push(token.parse().unwrap());
    });
    tokens
}


#[test]
fn test_squash() {
    assert_eq!(squash_whitespace("+     1    12"), "+ 1 12");
    assert_eq!(squash_whitespace("          + 15                    21"), "+ 15 21");
    assert_eq!(squash_whitespace("+ 12 51 "), "+ 12 51")
}

#[test]
fn test_tokenize() {
    assert_eq!(tokenize("+ 1 1"), vec!["+", "1", "1"]);
    assert_eq!(tokenize("     +     1     1     "), vec!["+", "1", "1"]);
}