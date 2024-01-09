use crate::freq;
use std::cmp::Ordering;

// Author: Willmo3
// A node represents either an internal node, with a left and right child,
// Or a leaf node, with a byte:contents frequency.
// To get the value of a node, descend left and right.

// CRUCIAL: Nodes must be normalized!
// Nodes are normalized by ordering their byte values from 0-256
// Representing their relative frequencies.
// If they are not normalized, and simply treated as a mapping from value -> frequency,
// their values may overflow and there may be ties!
#[derive(Debug)]
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
    // Return the sum of this node's counts.
    fn sum(&self) -> usize {
        match self {
            Node::Internal { left, right } =>  { left.sum() + right.sum() }
            Node::Leaf { contents } => contents.count()
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.sum() == other.sum()
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sum().cmp(&other.sum())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}