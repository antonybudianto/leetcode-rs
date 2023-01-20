struct Solution {}

const PHONE_MAP: [[char; 4]; 8] = [
    ['a', 'b', 'c', '-'],
    ['d', 'e', 'f', '-'],
    ['g', 'h', 'i', '-'],
    ['j', 'k', 'l', '-'],
    ['m', 'n', 'o', '-'],
    ['p', 'q', 'r', 's'],
    ['t', 'u', 'v', '-'],
    ['w', 'x', 'y', 'z'],
];

impl Solution {
    pub fn index_to_digit(s: &String, index: usize) -> usize {
        let digit = s.chars().nth(index).unwrap();
        let digit = digit.to_digit(10).unwrap() as usize;
        return digit - 2;
    }

    pub fn traverse_ch(v: &mut Vec<String>, sm: &String, s: &String, idx: usize, cidx: usize) {
        if idx + 1 > s.len() {
            v.push(sm.to_string());
        } else {
            let digit = Solution::index_to_digit(s, idx);
            let chrs = PHONE_MAP[digit];

            let ch = chrs[cidx];
            let mut sm2 = String::from(sm);
            sm2.push(ch);

            if idx + 1 < s.len() {
                let nx_digit = Solution::index_to_digit(s, idx + 1);
                let nx_chrs = PHONE_MAP[nx_digit];

                for i in 1..nx_chrs.len() {
                    if nx_chrs[i] != '-' {
                        Solution::traverse_ch(v, &sm2, s, idx + 1, i);
                    }
                }
            }

            Solution::traverse_ch(v, &sm2, s, idx + 1, 0);
        }
    }

    pub fn traverse(v: &mut Vec<String>, s: &String, index: usize) {
        let digit = Solution::index_to_digit(s, index);
        let chrs = PHONE_MAP[digit];

        for i in 0..chrs.len() {
            if chrs[i] != '-' {
                Solution::traverse_ch(v, &"".to_string(), s, 0, i);
            }
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let len = digits.len();
        let mut v = Vec::<String>::new();

        if len == 0 {
            return v;
        } else if len == 1 {
            for chr in digits.chars() {
                let chr = chr.to_digit(10).unwrap() as usize;
                let phone = PHONE_MAP[chr - 2];

                for ph in 0..phone.len() {
                    if phone[ph] != '-' {
                        v.push(phone[ph].to_string());
                    }
                }
            }
            return v;
        } else {
            // traverse from first number/char
            Solution::traverse(&mut v, &digits, 0);
            println!("RES:{:?}", v);
            return v;
        }
    }
}

fn main() {
    Solution::letter_combinations("23".to_string());
}
