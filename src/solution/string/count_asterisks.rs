struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut res = 0;
        let mut cnt = 0;
        let mut flag = false;

        for c in s.chars() {
            if c == '|' {
                if flag {
                    flag = false;
                } else {
                    res += cnt;
                    flag = true;
                }

                cnt = 0;
                continue;
            }

            if c == '*' {
                cnt += 1;
            }
        }

        res + cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let s = "l|*e*et|c**o|*de|".to_string();
        let expect = 2;
        assert_eq!(Solution::count_asterisks(s), expect);

        let s = "yo|uar|e**|b|e***au|tifu|l".to_string();
        let expect = 5;
        assert_eq!(Solution::count_asterisks(s), expect);

        let s = "*||||**||*||**|**||*|||**".to_string();
        let expect = 8;
        assert_eq!(Solution::count_asterisks(s), expect);
    }
}