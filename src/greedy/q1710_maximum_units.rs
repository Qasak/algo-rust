pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut boxes = box_types; let mut space = truck_size;
    boxes.sort_by(|a, b| b[1].cmp(&a[1]));
    let mut ret = 0;
    for b in boxes {
        if space <= b[0] {
            ret += b[1] * space;
            break;
        } else {
            space -= b[0];
            ret += b[1] * b[0];
        }
    }
    ret
}