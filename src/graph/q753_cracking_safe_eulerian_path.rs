use std::collections::HashSet;

pub fn crack_safe(n: i32, k: i32) -> String {

    fn dfs(node:i32, mo:i32, k:i32, seen:&mut HashSet<i32>, ans:&mut String){
        for x in 0..k {
            let mut e = node * 10 + x;
            if !seen.contains(&e) {
                seen.insert(e);
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
    dfs(0, mo, k, &mut seen, &mut ans);
    ans.push_str(&"0".repeat((n - 1) as usize));
    ans
}