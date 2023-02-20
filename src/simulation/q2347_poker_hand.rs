use std::collections::HashSet;
const RANKS_COUNT: usize = 14;
pub struct Hand {
    ranks: Vec<i32>,
    suits: Vec<char>,
    hand_type: HandType
}

#[derive(Debug, PartialEq)]
pub enum HandType {
    Flush,
    ThreeOfAKind,
    Pair,
    HighCard,
    Other
}
impl Hand {
    pub fn new(mut ranks: Vec<i32>, suits: Vec<char>) -> Self {
        ranks.sort();
        let hand_type = Hand::dispatch_type(&ranks, &suits);
        Self {
            ranks,
            suits,
            hand_type,
        }
    }

    fn dispatch_type(ranks: &[i32], suits: &[char]) -> HandType {
        match () {
            _ if Hand::is_flush(suits) => HandType::Flush,
            _ if Hand::is_n_of_a_kind(3, ranks) => HandType::ThreeOfAKind,
            _ if Hand::is_n_of_a_kind(2, ranks) => HandType::Pair,
            _ => HandType::HighCard
        }
    }

    fn get_type(&self) -> String {
        match self.hand_type {
            HandType::Flush => "Flush".to_owned(),
            HandType::ThreeOfAKind => "Three of a Kind".to_owned(),
            HandType::Pair => "Pair".to_owned(),
            _ => "High Card".to_owned(),
        }
    }

    fn is_flush(suits: &[char]) -> bool {
        let set: HashSet<_> = suits.iter().collect();
        set.len() == 1
    }

    fn is_n_of_a_kind(n: usize, ranks: &[i32]) -> bool {
        let mut cnt = vec![0; RANKS_COUNT];
        ranks.iter().any(|&r| {
            cnt[r as usize] += 1;
            cnt[r as usize] == n
        })
    }
}


pub fn best_hand(mut ranks: Vec<i32>, suits: Vec<char>) -> String {
    let hand = Hand::new(ranks, suits);
    hand.get_type()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_hand_flush() {
        let ranks = vec![3, 7, 8, 10, 13];
        let suits = vec!['d', 'd', 'd', 'd', 'd'];
        let hand_type = Hand::new(ranks, suits).hand_type;
        assert_eq!(hand_type, HandType::Flush);
    }

    #[test]
    fn test_best_hand_three_of_a_kind() {
        let ranks = vec![2, 2, 2, 4, 5];
        let suits = vec!['d', 'a', 'b', 'c', 'd'];
        let hand_type = Hand::new(ranks, suits).hand_type;
        assert_eq!(hand_type, HandType::ThreeOfAKind);
    }

    #[test]
    fn test_best_hand_pair() {
        let ranks = vec![2, 2, 3, 4, 5];
        let suits = vec!['d', 'a', 'b', 'c', 'd'];
        let hand_type = Hand::new(ranks, suits).hand_type;
        assert_eq!(hand_type, HandType::Pair);
    }

    #[test]
    fn test_best_hand() {
        assert_eq!(best_hand(vec![2, 3, 5, 7, 11], vec!['d', 'd', 'd', 'd', 'd']), "Flush".to_owned());
        assert_eq!(best_hand(vec![2, 2, 2, 7, 11], vec!['d', 'a', 'c', 'b', 'd']), "Three of a Kind".to_owned());
        assert_eq!(best_hand(vec![2, 3, 5, 7, 7], vec!['d', 'a', 'c', 'b', 'd']), "Pair".to_owned());
        assert_eq!(best_hand(vec![2, 3, 5, 7, 10], vec!['d', 'a', 'c', 'b', 'd']), "High Card".to_owned());
    }

}
