fn query(num:i32,limit:i32,vec:&Vec<i32>)->i32{
    let mut idx = 0;
    let mut count = 0;
    for i in (0..=14).rev(){
        let bit_num = ((num >> i) & 1) as usize;
        let bit_high = (limit >> i)&1;
        if bit_high == 1{
            let c = vec[idx*2+1+bit_num];
            count += c;
            idx = idx*2 + 2 - bit_num;
        }else{
            idx = idx*2+1+bit_num;
        }
    }
    return count;
}
fn insert(num:i32,vec:&mut Vec<i32>){
    let mut idx = 0;
    for i in (0..=14).rev(){
        let bit_num = ((num >> i) & 1) as usize;
        idx = idx*2+1+bit_num;
        vec[idx]+=1;
    }
}
pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
    let mut vec = vec![0_i32;1<<16];//left:0,right:1
    let mut ans = 0;
    insert(nums[0], &mut vec);
    for &num in nums.iter().skip(1){
        let a1 = query(num,high+1,&vec);
        let a2 = query(num,low,&vec);
        ans += a1-a2;
        insert(num,&mut vec);
    }

    return ans;
}
