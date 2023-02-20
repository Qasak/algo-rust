use std::collections::{BTreeMap, HashMap};

pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
    let mut arr: Vec<(&String, &String)> = key_name
        .iter()
        .zip(key_time.iter())
        .map(|(x, y)| (x, y))
        .collect();

    arr.sort_by(|a, b| a.1.cmp(&b.1));
    let mut map = HashMap::new();
    for (k, v) in arr {
        let mut entry = map.entry(k).or_insert(Vec::new());
        entry.push(v);
    }

    let mut ret: Vec<String> = vec![];
    for (k, v) in map.iter() {
        let n = v.len();
        if n < 3 {
            continue;
        }
        let mut iter = v.iter();
        let mut first = iter.next().unwrap();
        let mut second = iter.next().unwrap();
        for third in iter {
            if time_cmp(first, third) {
                ret.push(k.to_string());
                break;
            }
            first = second;
            second = third;
        }
    }
    ret.sort();
    ret
}

fn time_cmp(t1: &str, t2: &str) -> bool {
    let m1 = hour_to_min(t1);
    let m2 = hour_to_min(t2);
    m2 - m1 <= 60
}

fn hour_to_min(t: &str) -> i32 {
    t[0..2].parse::<i32>().unwrap() * 60 + t[3..5].parse::<i32>().unwrap()
}

#[cfg(test)]
mod test {
    use crate::simulation::q1604_time_check::alert_names;

    #[test]
    fn f1() {
        let key_name = vec![
            "daniel".to_string(),
            "daniel".to_string(),
            "daniel".to_string(),
            "luis".to_string(),
            "luis".to_string(),
            "luis".to_string(),
            "luis".to_string(),
        ];
        let key_time = vec![
            "10:00".to_string(),
            "10:40".to_string(),
            "11:00".to_string(),
            "09:00".to_string(),
            "11:00".to_string(),
            "13:00".to_string(),
            "15:00".to_string(),
        ];
        let ret = alert_names(key_name, key_time);
        println!("1 {:?}", ret);
    }

    #[test]
    fn f2() {
        let key_name = vec![
            "alice".to_string(),
            "alice".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            "bob".to_string(),
            "bob".to_string(),
            "bob".to_string(),
        ];
        let key_time = vec![
            "12:01".to_string(),
            "12:00".to_string(),
            "18:00".to_string(),
            "21:00".to_string(),
            "21:20".to_string(),
            "21:30".to_string(),
            "23:00".to_string(),
        ];
        let ret = alert_names(key_name, key_time);
        println!("2 {:?}", ret);
    }

    #[test]
    fn f3() {
        let key_name = vec![
            "leslie".to_string(),
            "leslie".to_string(),
            "leslie".to_string(),
            "clare".to_string(),
            "clare".to_string(),
            "clare".to_string(),
            "clare".to_string(),
        ];
        let key_time = vec![
            "13:00".to_string(),
            "13:20".to_string(),
            "14:00".to_string(),
            "18:00".to_string(),
            "18:51".to_string(),
            "19:30".to_string(),
            "19:49".to_string(),
        ];
        let ret = alert_names(key_name, key_time);
        println!("3 {:?}", ret);
    }
}
