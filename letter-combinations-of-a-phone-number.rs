use std::collections::HashMap;
struct Solution {}
impl Solution {
    pub fn get_chars(map: &HashMap<i8, [char; 4]>, i: i8) -> Option<[char; 4]> {
        match map.get(&i) {
            Some(phone) => {
                return Some(*phone);
            }
            None => None,
        }
    }

    pub fn index_to_digit(s: &String, index: usize) -> i8 {
        let digit = s.chars().nth(index as usize).unwrap();
        let digit = digit.to_digit(10).unwrap() as i8;
        digit
    }

    pub fn traverse_ch(
        v: &mut Vec<String>,
        m: &HashMap<i8, [char; 4]>,
        sm: &String,
        s: &String,
        idx: usize,
        cidx: usize,
    ) {
        if idx + 1 > s.len() {
            v.push(sm.to_string());
        } else {
            let digit = Solution::index_to_digit(s, idx);
            let chrs = Solution::get_chars(m, digit).unwrap();

            let ch = chrs[cidx];
            let mut sm2 = String::from(sm);
            sm2.push(ch);

            if idx + 1 < s.len() {
                let nx_digit = s.chars().nth(idx + 1).unwrap();
                let nx_digit = nx_digit.to_digit(10).unwrap() as i8;
                let nx_chrs = Solution::get_chars(m, nx_digit).unwrap();
                for i in 1..nx_chrs.len() {
                    if nx_chrs[i] != '-' {
                        Solution::traverse_ch(v, m, &sm2, s, idx + 1, i);
                    }
                }
            }

            Solution::traverse_ch(v, m, &sm2, s, idx + 1, 0);
        }
    }

    pub fn traverse(v: &mut Vec<String>, m: &HashMap<i8, [char; 4]>, s: &String, index: u32) {
        let digit = Solution::index_to_digit(s, index as usize);
        let chrs = Solution::get_chars(m, digit).unwrap();

        for i in 0..chrs.len() {
            if chrs[i] != '-' {
                Solution::traverse_ch(v, &m, &"".to_string(), s, 0, i);
            }
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let phone_map: HashMap<i8, [char; 4]> = HashMap::from([
            (2, ['a', 'b', 'c', '-']),
            (3, ['d', 'e', 'f', '-']),
            (4, ['g', 'h', 'i', '-']),
            (5, ['j', 'k', 'l', '-']),
            (6, ['m', 'n', 'o', '-']),
            (7, ['p', 'q', 'r', 's']),
            (8, ['t', 'u', 'v', '-']),
            (9, ['w', 'x', 'y', 'z']),
        ]);

        let len = digits.len();
        let mut v = Vec::<String>::new();

        if len == 0 {
            return v;
        } else if len == 1 {
            for chr in digits.chars() {
                let chr = chr.to_digit(10).unwrap() as i8;
                match phone_map.get(&chr) {
                    Some(phone) => {
                        for ph in 0..phone.len() {
                            if phone[ph] != '-' {
                                v.push(phone[ph].to_string());
                            }
                        }
                    }
                    None => (),
                }
            }
            return v;
        } else {
            // traverse from first number/char
            Solution::traverse(&mut v, &phone_map, &digits, 0);
            println!("RES:{:?}", v);
            return v;
        }
    }
}

fn main() {
    Solution::letter_combinations("23".to_string());
}
