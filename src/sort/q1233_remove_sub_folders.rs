// use regex::internal::Input;
use std::collections::HashSet;

pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
    let mut set = HashSet::new();
    folder.sort();
    folder.into_iter().for_each(|path| {
        for i in 1..path.len() {
            if &path[i..i + 1] == "/" {
                if set.contains(&path[0..i]) {
                    return;
                }
            }
        }
        set.insert(path);
    });
    set.into_iter().collect::<Vec<String>>()
}

pub fn remove_subfolders_1(mut folder: Vec<String>) -> Vec<String> {
    folder.sort();
    let mut ret: Vec<String> = vec![];
    folder.into_iter().for_each(|path| {
        if let Some(pre) = ret.last() {
            let m = pre.len();
            let n = path.len();
            if n <= m || !(pre == &path[0..m] && &path[m..m + 1] == "/") {
                ret.push(path.into());
            }
        } else {
            ret.push(path.into());
        }
    });
    ret
}
