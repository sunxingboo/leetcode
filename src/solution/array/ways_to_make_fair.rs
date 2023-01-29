struct Solution;

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        // left的0、1分别代表某一个索引位置左侧的奇、偶索引值的和；right同理
        let mut left = (0, 0);
        let mut right = (0, 0);

        // 先统计总和
        for (i, num) in nums.iter().enumerate() {
            if i & 1 > 0 {
                right.0 += num;
            } else {
                right.1 += num;
            }
        }

        // 再从头遍历每一个位置
        for (i, num) in nums.iter().enumerate() {
            // 从right中减去对应索引的值
            if i & 1 > 0 {
                right.0 -= num;
            } else {
                right.1 -= num;
            }

            // 若删除当前索引，right中的奇、偶类型会调换
            if right.0 + left.1 == right.1 + left.0 {
                res += 1;
            }

            // 将当前值加到left中
            if i & 1 > 0 {
                left.0 += num;
            } else {
                left.1 += num;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let nums = vec![2, 1, 6, 4];
        let except = 1;
        assert_eq!(Solution::ways_to_make_fair(nums), except);

        let nums = vec![1, 1, 1];
        let except = 3;
        assert_eq!(Solution::ways_to_make_fair(nums), except);

        let nums = vec![1, 2, 3];
        let except = 0;
        assert_eq!(Solution::ways_to_make_fair(nums), except);
    }
}