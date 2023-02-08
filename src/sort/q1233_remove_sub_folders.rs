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