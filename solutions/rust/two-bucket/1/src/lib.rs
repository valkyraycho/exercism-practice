use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let (b1, b2) = match start_bucket {
        Bucket::One => (capacity_1, 0),
        Bucket::Two => (0, capacity_2),
    };

    let forbidden = match start_bucket {
        Bucket::One => (0, capacity_2),
        Bucket::Two => (capacity_1, 0),
    };

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((b1, b2, 1u8));
    visited.insert((b1, b2));

    while let Some((b1, b2, moves)) = queue.pop_front() {
        if b1 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: b2,
            });
        }

        if b2 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: b1,
            });
        }

        let transfer_from_b1_to_b2 = b1.min(capacity_2 - b2);
        let transfer_from_b2_to_b1 = b2.min(capacity_1 - b1);

        let next_states = [
            (capacity_1, b2),
            (b1, capacity_2),
            (0, b2),
            (b1, 0),
            (b1 - transfer_from_b1_to_b2, b2 + transfer_from_b1_to_b2),
            (b1 + transfer_from_b2_to_b1, b2 - transfer_from_b2_to_b1),
        ];

        for (next_b1, next_b2) in next_states {
            if (next_b1, next_b2) != forbidden && !visited.contains(&(next_b1, next_b2)) {
                visited.insert((next_b1, next_b2));
                queue.push_back((next_b1, next_b2, moves + 1));
            }
        }
    }

    None
}
