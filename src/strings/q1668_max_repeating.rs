// use .contains(), rust use [Two-Way Algorithm](https://en.wikipedia.org/wiki/Two-way_string-matching_algorithm) to implement it.
// it can be viewed as a combination of the forward-going Knuth–Morris–Pratt algorithm (KMP) and the backward-running Boyer–Moore string-search algorithm (BM)
// time: O(n)
// https://github.com/rust-lang/rust/blob/946a88a989acdcc3b0d05a666eaac0db414ec2cd/library/core/src/str/pattern.rs#L1226
pub fn max_repeating(sequence: String, word: String) -> i32 {
    let (mut cnt, mut ret, mut cur) = (1, 0, String::from(word.as_str()));
    while cur.len() <= sequence.len() {
        if sequence.contains(&cur) {
            ret = cnt;
        }
        cur.push_str(word.as_str());
        cnt += 1;
    }
    ret
}

// bf: handwritten version O(nm) contains
pub fn max_repeating_1(sequence: String, word: String) -> i32 {
    fn contains(a: &String, b: &String) -> bool {
        if a.len() < b.len() {
            return contains(b, a);
        }
        let (mut it1, mut it2) = (a.chars().skip(0), b.chars());
        let mut skip = 0;
        loop {
            match (it1.next(), it2.next()) {
                (Some(ch1), Some(ch2)) => if ch1 != ch2 { skip += 1; it1 = a.chars().skip(skip);it2 = b.chars()} ,
                (_, None) => return true,
                (None, _) => return false
            }
        }
    }
    let (mut cnt, mut ret, mut cur) = (1, 0, String::from(word.as_str()));
    while cur.len() <= sequence.len() {
        if contains(&sequence, &cur) {
            ret = cnt;
        }
        cur.push_str(word.as_str());
        cnt += 1;
    }
    ret
}

// dp: O(nm)
pub fn max_repeating_2(sequence: String, word: String) -> i32 {
    let (n, m) = (sequence.len(), word.len());
    // f[i]: The maximum repeat value of `sequence[0..i]` / [0, i)
    // if m length slice of seq equal to word, f[i] = f[i - m] + 1
    let mut f = vec![0; n + 1];
    for i in m..=n {
        // sequence[0..m) to sequence[(n - m)..n)
        if &sequence.as_bytes()[(i - m)..i] == word.as_bytes() {
            f[i] = f[i - m] + 1;
        }
    }
    *f.iter().max().unwrap()
}


// O(m + n) Rabin-Karp
// The bottleneck of dp solution is that it takes O(m) complexity to compare the strings each time.
// This process can be optimised with 'string hashing' ：the Rabin-Karp algo



// O(m + n) kmp





