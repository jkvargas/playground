use std::collections::HashMap;

struct Twitter {
    posts: Vec<Tweet>,
    follows: HashMap<i32, HashMap<i32, bool>>,
}

struct Tweet {
    user_id: i32,
    tweet_id: i32,
}

impl Tweet {
    fn new(user_id: i32, tweet_id: i32) -> Self {
        Self { user_id, tweet_id }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Twitter {
            posts: Vec::new(),
            follows: HashMap::new(),
        }
    }

    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.posts.push(Tweet::new(user_id, tweet_id));
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut count = 0;

        for i in (0..self.posts.len()).rev() {
            if count == 10 {
                break;
            }

            if self.posts[i].user_id == user_id || self.does_follow(user_id, self.posts[i].user_id)
            {
                res.push(self.posts[i].tweet_id);
                count += 1;
            }
        }

        res
    }

    #[inline]
    fn does_follow(&self, user_id: i32, post: i32) -> bool {
        if let Some(f) = self.follows.get(&user_id) {
            if let Some(t) = f.get(&post) {
                return *t;
            }
        }

        false
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(f) = self.follows.get_mut(&follower_id) {
            if let Some(s) = f.get_mut(&followee_id) {
                *s = true;
            } else {
                f.insert(followee_id, true);
            }
        } else {
            let mut hash = HashMap::new();
            hash.insert(followee_id, true);
            self.follows.insert(follower_id, hash);
        }
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(f) = self.follows.get_mut(&follower_id) {
            if let Some(s) = f.get_mut(&followee_id) {
                *s = false;
            }
        }
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */

// #[test]
// fn test_one() {
fn main() {
    let mut twitter = Twitter::new();

    twitter.post_tweet(1, 4);
    twitter.post_tweet(2, 5);
    twitter.post_tweet(1, 2);
    twitter.unfollow(1, 2);
    twitter.get_news_feed(1);
}
