// Andrew's monotone chain convex hull algorithm
// https://algorithmist.com/wiki/Monotone_chain_convex_hull

use std::collections::VecDeque;
type Tree = Vec<i32>;

pub fn outer_trees(mut trees: Vec<Tree>) -> Vec<Tree> {
    fn cross(B: &Tree, A: &Tree, T: &Tree) -> i32{
        return (T[1]-B[1])*(B[0]-A[0]) - (B[1]-A[1])*(T[0]-B[0]);
    }

    trees.sort();

    let mut F: VecDeque<Tree> = VecDeque::new();

    for T in trees {
        while F.len() >= 2 && cross(&F[F.len()-1],&F[F.len()-2], &T) < 0 {
            F.pop_back();
        }
        F.push_back(T.clone());
        while F.len() >= 2 && cross(&F[0],&F[1], &T) > 0 {
            F.pop_front();
        }
        F.push_front(T.clone());
    }

    let mut fence = Vec::from(F);
    fence.sort_unstable();
    fence.dedup();
    return fence;
}
