pub fn maximum69_number(num: i32) -> i32 {
    let mut s = num.to_string().chars().collect::<Vec<char>>();
    for i in 0..s.len() {
        if s[i] == '6' {
            s[i] = '9';
            break;
        }
    }
    s.into_iter().collect::<String>().parse::<i32>().unwrap()
}

pub fn maximum69_number_1(num: i32) -> i32 {
    num.to_string()
        .replacen('6', "9", 1)
        .parse::<i32>()
        .unwrap()
}

pub fn maximum69_number_2(num: i32) -> i32 {
    let mut mask = 1000;
    while mask > 0 {
        if num % (mask * 10) / mask == 6 {
            return num + mask * 3;
        }
        mask /= 10;
    }
    num
}
