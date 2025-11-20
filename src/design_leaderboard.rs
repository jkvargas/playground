// https://leetcode.com/problems/design-a-leaderboard/description/?envType=company&envId=bloomberg&favoriteSlug=bloomberg-thirty-days

use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
    iter::FromIterator,
};

#[derive(Eq, Ord)]
struct Player {
    id: i32,
    score: i32,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Player {
    fn new(id: i32, score: i32) -> Self {
        Player { id, score }
    }
}

impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

struct Leaderboard {
    players: HashSet<Player>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Leaderboard {
    fn new() -> Self {
        Self {
            players: HashSet::new(),
        }
    }

    fn add_score(&mut self, player_id: i32, score: i32) {
        let player = Player::new(player_id, score);

        if let Some(mut from_collection) = self.players.take(&player) {
            from_collection.score += score;
            self.players.insert(from_collection);
        } else {
            self.players.insert(player);
        }
    }

    fn top(&self, k: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(self.players.iter());

        let mut result = 0;
        for i in 0..k {
            result += heap.pop().unwrap().score;
        }

        result
    }

    fn reset(&mut self, player_id: i32) {
        self.players.remove(&Player::new(player_id, 0));
    }
}

#[cfg(test)]
mod tests {
    use crate::design_leaderboard::Leaderboard;

    #[test]
    fn test_one() {
        let mut leaderboard = Leaderboard::new();
        leaderboard.add_score(1, 73);
        leaderboard.add_score(2, 56);
        leaderboard.add_score(3, 39);
        leaderboard.add_score(4, 51);
        leaderboard.add_score(5, 4);
        assert_eq!(73, leaderboard.top(1));
        leaderboard.reset(1);
        leaderboard.reset(2);
        leaderboard.add_score(2, 51);
        assert_eq!(141, leaderboard.top(3));
    }
}
