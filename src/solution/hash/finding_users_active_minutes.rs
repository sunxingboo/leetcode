use std::collections::HashMap;

pub struct Solution{}

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut m = HashMap::new();

        for log in logs {
            let (user_id, time) = (log[0], log[1]);

            let mut minutes = m.entry(user_id).or_insert(HashMap::new());

            minutes.insert(time, true);
        }

        let mut res = vec![0; k as usize];
        for value in m.values() {
            res[value.len() - 1] += 1;
        }

        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let logs = vec![vec![0,5],vec![1,2],vec![0,2],vec![0,5],vec![1,3]];
        let k = 5;
        let expect = vec![0, 2, 0, 0, 0];
        assert_eq!(Solution::finding_users_active_minutes(logs, k), expect);

        let logs = vec![vec![1,1],vec![2,2],vec![2,3]];
        let k = 4;
        let expect = vec![1, 1, 0, 0];
        assert_eq!(Solution::finding_users_active_minutes(logs, k), expect);
    }
}