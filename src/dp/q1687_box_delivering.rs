// WA at :
// [[2,4],[2,5],[3,1],[3,2],[3,7],[3,1],[4,4],[1,3],[5,2]]
// 5
// 5
// 7
pub fn box_delivering(boxes: Vec<Vec<i32>>, ports_count: i32, max_boxes: i32, max_weight: i32) -> i32 {
    let n = boxes.len();
    // f[i][0]: 第i次回码头，box数量c，box重量w重新计数
    // f[i][1]: 第i次不回码头，当前box已经在车上
    let mut f = vec![vec![(i32::MAX, max_boxes, max_weight); 2]; n];
    f[0][0] = (2, max_boxes, max_weight);
    f[0][1] = (1, max_boxes - 1, max_weight - boxes[0][1]);
    for i in 1..n {
        let pp = boxes[i - 1][0];
        let p = boxes[i][0];
        let w = boxes[i][1];
        let (res0, c0, w0) = f[i - 1][0];
        let (res1, c1, w1) = f[i - 1][1];
        let t00 =
            (res0 + 2, max_boxes, max_weight);
        let t10 = if c1 >= 1 && w1 >= w {
            (res1 + if p == pp {1} else {2}, max_boxes, max_weight)
        } else {
            t00
        };
        let t01 =
            (res0 + 1, max_boxes - 1, max_weight - w);
        let t11 = if c1 >= 1 && w1 >= w {
            (res1 + if p == pp {0} else {1}, c1 - 1, w1 - w)
        } else {
            t01
        };
        f[i][0] = if t00.0 != t10.0 {t00.min(t10)} else {t00};
        f[i][1] = if t01.0 != t11.0 {t01.min(t11)} else {t01};
    }
    println!("{:?}", &f);
    f[n - 1][0].0.min(f[n - 1][1].0 + 1)
}