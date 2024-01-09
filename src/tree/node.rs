use crate::freq;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

// Author: Willmo3
// A node represents either an internal node, with a left and right child,
// Or a leaf node, with a byte:contents frequency.
// To get the value of a node, descend left and right.

// CRUCIAL: Nodes must be normalized!
// Nodes are normalized by ordering their byte values from 0-256
// Representing their relative frequencies.
// If they are not normalized, and simply treated as a mapping from value -> frequency,
// their values may overflow and there may be ties!
pub enum Node {
    Internal { left: Box<Node>, right: Box<Node> },
    Leaf { contents: freq::FreqCount },
}

/// CONSTRUCTORS
pub fn leaf(contents: freq::FreqCount) -> Node {
    Node::Leaf { contents }
}
pub fn internal(left: Box<Node>, right: Box<Node>) -> Node {
    Node::Internal { left, right }
}

/// INSTANCE METHODS
impl Node {
    /// Return the sum of this node's counts.
    fn sum(&self) -> usize {
        match self {
            Node::Internal { left, right } =>  { left.sum() + right.sum() }
            Node::Leaf { contents } => contents.count()
        }
    }

    /// Return the minimum byte in this node.
    /// This is used as a tiebreaker.
    fn min_byte(&self) -> u8 {
        match self {
            Node::Leaf { contents} => { contents.byte() }
            Node::Internal { left, right} => {
                if left.min_byte() < right.min_byte() {
                    left.min_byte()
                } else {
                    right.min_byte()
                }
            }
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.sum() == other.sum()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Internal { left, right } => {
                let left = left.fmt(f);
                let right = right.fmt(f);
                if left.is_err() {
                    left
                } else if right.is_err() {
                    right
                } else {
                    Ok(())
                }
            }
            Node::Leaf { contents } => {
                f.write_fmt(format_args!("{}: {} ", contents.byte(), contents.count()))
            }
        }
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // min byte used to ensure that ties are broken consistently.
        self.sum().cmp(&other.sum())
            .then_with(|| self.min_byte().cmp(&other.min_byte()))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}