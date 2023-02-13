mod q239_max_sliding_window;
mod q1234_balanced_string;

pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    fn f(c: char, cs: &Vec<char>, k: i32) -> i32 {
        let n = cs.len();
        let (mut cnt, mut j, mut ret) = (0, 0, 0);
        for i in 0..n {
            if cs[i] == c {cnt += 1;}
            while j < n && cnt > k {
                if cs[j] == c {cnt -= 1;}
                j += 1;
            }
            ret = ret.max(i - j + 1);
        }
        ret as i32
    }
    let cs = answer_key.chars().collect::<Vec<char>>();
    f('T', &cs, k).max(f('F', &cs, k))
}