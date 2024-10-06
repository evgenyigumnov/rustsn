use regex::Regex;

pub fn remove_comments(text: &str) -> String {
    let re_comment = Regex::new(r"(?m)^#.*$").unwrap();
    re_comment
        .replace_all(text, "")
        .to_string()
        .trim()
        .to_string()
}
