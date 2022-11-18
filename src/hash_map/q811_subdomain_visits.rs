use std::collections::HashMap;

pub fn subdomain_visits(s: Vec<String>) -> Vec<String> {
    let mut map = HashMap::new();
    for w in s {
        let i = w.find(" ").unwrap();
        let (cnt, domain) = (w[..i].parse::<i32>().unwrap(), w[i + 1..].to_string());
        // insert subdomains' count
        for (j, ch) in domain.as_bytes().into_iter().enumerate() {
            if ch == &b'.' {
                if let Some(mut v) = map.get_mut(&domain[j + 1..]) {
                    *v += cnt;
                } else {
                    map.insert(domain[j + 1..].to_string(), cnt);
                }

            }
        }
        // insert main domain's count
        if let Some(mut v) = map.get_mut(&domain) {
            *v += cnt;
        } else {
            map.insert(domain, cnt);
        }
    }
    map.into_iter().map(|(i, v)| format!("{} {}", v, i)).collect::<Vec<_>>()
}
pub fn subdomain_visits_1(cpdomains: Vec<String>) -> Vec<String> {
    use std::collections::HashMap;
    let mut cnt_cache = HashMap::new();
    for cpdomain in cpdomains {
        let i = cpdomain.find(" ").unwrap();
        let (cnt, domain) = (cpdomain[..i].parse::<i32>().unwrap(), cpdomain[i + 1..].to_string());

        for (j, ch) in domain.as_bytes().into_iter().enumerate() {
            if ch == &b'.' { *cnt_cache.entry((&domain[j + 1..]).to_string()).or_insert(0) += cnt; }
        }
        *cnt_cache.entry(domain).or_insert(0) += cnt;
    }
    cnt_cache.into_iter().map(|(i, v)| format!("{} {}", v, i)).collect::<Vec<_>>()
}

pub fn subdomain_visits_2(s: Vec<String>) -> Vec<String> {
    let mut map = HashMap::new();
    let mut cnt = 0;
    for ss in s.iter() {
        for (i, c) in ss.as_bytes().iter().enumerate() {
            if *c == b' ' {
                cnt = ss[..i].parse().unwrap();
                map.entry(&ss[i + 1..]).and_modify(|c| *c += cnt).or_insert(cnt);
                continue;
            }
            if *c == b'.'{
                map.entry(&ss[i + 1..]).and_modify(|c| *c += cnt).or_insert(cnt);
            }
        }
    }
    map.into_iter().map(|(k, v)| {
        format!("{} {}", v, k)
    }).collect()
}
