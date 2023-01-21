struct Solution {}

impl Solution {
    pub fn bsearch(nums: &Vec<i32>, res: &[i32; 2], target: i32, li: usize, ri: usize) -> [i32; 2] {
        let l = *nums.get(li).unwrap();
        let r = *nums.get(ri).unwrap();

        // when left and right mets...
        if li >= ri {
            if l as i32 == target {
                return [li as i32, ri as i32];
            }
            return [-1, -1];
        }

        // finally find the pair
        if l == target && r == target {
            return [li as i32, ri as i32];
        }

        // 5,7,7,8,8,10
        // if target is lower than left, then no way to find...
        if l > target {
            return [-1, -1];
        }

        // 5,7,7,8,8,10
        //       ^    ^
        if l == target {
            return Solution::bsearch(nums, res, target, li, ri - 1);
        } else if r == target {
            return Solution::bsearch(nums, res, target, li + 1, ri);
        }

        return Solution::bsearch(nums, res, target, li + 1, ri - 1);
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        if len == 0 {
            return Vec::from([-1, -1]);
        }

        let res = Solution::bsearch(&nums, &[-1, -1], target, 0, nums.len() - 1);
        let v = Vec::<i32>::from(res);
        return v;
    }
}

fn main() {
    let v = Vec::from([1]);
    Solution::search_range(v, 0);
}
