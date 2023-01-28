use std::collections::HashSet;

pub fn crack_safe(n: i32, k: i32) -> String {

    fn dfs(node:i32, mo:i32, k:i32, seen:&mut HashSet<i32>, ans:&mut String){
        for x in 0..k {
            let mut e = node * 10 + x;
            if !seen.contains(&e) {
                seen.insert(e);
                // e % mo: 取后面n - 1个数字
                dfs(e % mo, mo, k,  seen,  ans);
                ans.push((x as u8 + b'0') as char);
                // println!("{:?}",ans);
                // println!("{:?}",seen);
            }
        }
    }

    let mut seen = HashSet::new();
    let mut ans = String::new();
    let mo = 10_i32.pow(n as u32 - 1);
    // println!("{}", mo);
    // println!("{}", 12315 % mo);
    dfs(0, mo, k, &mut seen, &mut ans);
    ans.push_str(&"0".repeat((n - 1) as usize));

    ans
}

#[cfg(test)]
mod test {
    use crate::graph::q753_cracking_safe_eulerian_path::crack_safe;

    #[test]
    fn f() {
        let s = crack_safe(5, 3);
        println!("{}", s);
        assert_eq!("0000222221221212221121211221112111112222012220212201122022120121202112011120220201202022210122102121011210202102211012110211101111020110220101201021010110102220012200212001120020200102002210012100211001110020100101002200012000210001100020000100000".to_string(), s);
    }
}