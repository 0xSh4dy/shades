#[allow(dead_code)]
pub fn remove_whitespace(s:&String)->String{
    return s.chars().filter(|x|!x.is_whitespace()).collect();
}