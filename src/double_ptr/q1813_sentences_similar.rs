// naive double ptr
pub fn are_sentences_similar(s1: String, s2: String) -> bool {
    if s1.len() < s2.len() {
        return Self::are_sentences_similar(s2, s1);
    }
    let v1 = s1.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
    let v2 = s2.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
    let (n, m) = (v1.len(), v2.len());
    if m == 1 {
        return v2[0] == v1[0] || v2[0] == v1[n - 1];
    }
    let (mut i, mut j, mut ii, mut jj) = (0, n - 1, 0, m - 1);
    while ii < jj {

        while ii < jj && v1[i] == v2[ii] {
            i += 1; ii += 1;
        }
        while ii < jj && v1[j] == v2[jj] {
            j -= 1; jj -= 1;
        }
        if v1[i] != v2[ii] || v1[j] != v2[jj] {
            break;
        }
    }

    ii == jj && (v1[i] == v2[ii] || v1[j] == v2[jj])
}
