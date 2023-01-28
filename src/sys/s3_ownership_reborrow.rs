#[test]
fn test_mut_ref_move() {
    let mut s = 3;
    let p = &mut s;
    // borrow of moved value: `p`
    // let x = p;
    // println!("{}", p);
    *p = 2;
    let x = &*p;
    println!("{}", x);

}

#[test]
fn test_reborrow() {
    let mut data = vec![1, 2, 3, 4];
    let b = &mut data;
    println!("sum:{}", sum(b));
    print!("{:?}", b);

}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}