use std::collections::BinaryHeap;
use std::cmp::Reverse;
// struct Order {
//     price: Price,
//     amount: Amount,
//     order_type: OrderType
// }

// #[derive(Debug, Copy, Clone)]
// struct Price(i32);

// #[derive(Debug, Copy, Clone)]
// struct Amount(i32)

// #[derive(Debug)]
// enum OrderType {
//     Buy: 0,
//     Sell: 1
// }
pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
    let mo = 1e9 as i32 + 7;
    // q_buy: big
    // q_sell: small
    let (mut q_buy, mut q_sell): (BinaryHeap<(i32, i32)>, BinaryHeap<(Reverse<i32>, i32)>) = (BinaryHeap::new(), BinaryHeap::new());
    for item in orders {
        let (price, mut amount, order_type) = (item[0], item[1], item[2]);
        // 0 == buy
        if order_type == 0 {
            if q_sell.is_empty() || q_sell.peek().unwrap().0.0 > price {
                q_buy.push((price, amount));
            } else {
                while let Some((Reverse(sp), sa)) = q_sell.pop() {
                    if sp > price {
                        q_sell.push((Reverse(sp), sa));
                        break;
                    }
                    if sa > amount {
                        q_sell.push((Reverse(sp), sa - amount));
                        amount = 0;
                        break;
                    } else {
                        amount -= sa;
                    }
                }
                if amount > 0 {
                    q_buy.push((price, amount));
                }
            }
            // 1 == sell
        } else {
            if q_buy.is_empty() || q_buy.peek().unwrap().0 < price {
                q_sell.push((Reverse(price), amount));
            } else {
                while let Some((bp, ba)) = q_buy.pop() {
                    if bp < price {
                        q_buy.push((bp, ba));
                        break;
                    }
                    if ba > amount {
                        q_buy.push((bp, ba - amount));
                        amount = 0;
                        break;
                    } else {
                        amount -= ba;
                    }
                }
                if amount > 0 {
                    q_sell.push((Reverse(price), amount));
                }
            }
        }
    }



    let mut ret = 0;
    while let Some((Reverse(p), a)) = q_sell.pop() {
        ret += a;
        ret %= mo;
    }
    while let Some((p, a)) = q_buy.pop() {
        ret += a;
        ret %= mo;
    }
    ret
}
