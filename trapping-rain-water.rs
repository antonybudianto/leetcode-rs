struct Solution {}

/**
 * Got TLE at 320/322... WIP
 */

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        if len == 0 {
            return 0;
        }
        let mut max = *height.get(0).unwrap();
        for index in 1..len {
            let val = *height.get(index).unwrap();
            if val > max {
                max = val;
            }
        }
        // println!("MAX:{max}");

        let mut count = 0;
        for index in 0..max {
            let mut l: i32 = -1;
            for j in 0..len {
                let sign = *height.get(j).unwrap();
                if sign >= max - index {
                    print!("⬛️");
                    if l == -1 {
                        l = j as i32;
                    } else {
                        count = count + (j as i32 - l - 1);
                        l = j as i32;
                    }
                } else {
                    print!("⬜️");
                }
            }
            println!("C:{count}");
        }

        println!("RES:{count}");
        return count;
    }
}

fn main() {
    Solution::trap(Vec::from([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
}
