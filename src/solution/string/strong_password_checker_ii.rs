pub struct Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }

        let special = "!@#$%^&*()-+";
        let mut special_count = 0;
        let mut big_char_count = 0;
        let mut small_char_count = 0;
        let mut number_count = 0;
        let mut last_char = ' ';

        for ch in password.chars() {
            if special.contains(ch) {
                special_count += 1;
            }

            if ch >= '0' && ch <= '9' {
                number_count += 1;
            }

            if ch >= 'A' && ch <= 'Z' {
                big_char_count += 1;
            }

            if ch >= 'a' && ch <= 'z' {
                small_char_count += 1;
            }

            if ch == last_char {
                return false;
            }

            last_char = ch;
        }

        special_count > 0 && big_char_count > 0 && small_char_count > 0 && number_count > 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let password = "IloveLe3tcode!".to_string();
        let expect = true; 
        assert_eq!(Solution::strong_password_checker_ii(password), expect);

        let password = "Me+You--IsMyDream".to_string();
        let expect = false; 
        assert_eq!(Solution::strong_password_checker_ii(password), expect);

        let password = "1aB".to_string();
        let expect = false; 
        assert_eq!(Solution::strong_password_checker_ii(password), expect);
    }
}