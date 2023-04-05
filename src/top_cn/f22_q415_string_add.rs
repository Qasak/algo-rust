pub fn add_strings(num1: String, num2: String) -> String {
    if num1.len() < num2.len() {
        return add_strings(num2, num1);
    }
    let mut carry = 0;
    let cs1 = num1.chars().collect::<Vec<char>>();
    let cs2 = num2.chars().collect::<Vec<char>>();
    let (mut n, mut m) = (num1.len(), num2.len());
    let mut s = vec![];
    while m > 0 {
        m -= 1;
        n -= 1;
        let cur = carry + cs1[n].to_digit(10).unwrap() + cs2[m].to_digit(10).unwrap();
        s.push(char::from_digit(cur % 10, 10).unwrap());
        carry = cur / 10;
    }
    while n > 0 {
        n -= 1;
        let cur = carry + cs1[n].to_digit(10).unwrap();
        s.push(char::from_digit(cur % 10, 10).unwrap());
        carry = cur / 10;
    }
    if carry == 1 {
        s.push('1');
    }
    s.iter().rev().collect()
}
