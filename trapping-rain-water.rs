struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut left = 0;
        let mut right = len - 1;
        let mut l_max = 0;
        let mut r_max = 0;
        let mut res = 0;

        while left < right {
            let a = *height.get(left).unwrap();
            let b = *height.get(right).unwrap();
            l_max = std::cmp::max(l_max, a);
            r_max = std::cmp::max(r_max, b);

            if l_max < r_max {
                res += l_max - a;
                left += 1;
            } else {
                res += r_max - b;
                right -= 1;
            }
        }

        return res;
    }
}

fn main() {
    Solution::trap(Vec::from([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
}
