use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use crate::encoding::bitsequence::BitSequence;
use crate::ordering::byteordering::ByteOrdering;
use crate::tree::node::Node::{Internal, Leaf};

// Author: Will Morris
// A node represents either an internal node, with a left and right child,
// Or a leaf node, with a byte:contents frequency.
// To get the value of a node, descend left and right.

// CRUCIAL: Nodes must be normalized!
// Nodes are normalized by ordering their byte values from 0-256
// Representing their relative frequencies.
// If they are not normalized, and simply treated as a mapping from value -> frequency,
// their values may overflow and there may be ties!
#[derive(Hash)]
pub enum Node {
    Internal { left: Box<Node>, right: Box<Node> },
    Leaf { contents: ByteOrdering },
}

/// CONSTRUCTORS
pub fn leaf(contents: ByteOrdering) -> Node {
    Leaf { contents }
}
pub fn internal(left: Box<Node>, right: Box<Node>) -> Node {
    Internal { left, right }
}

/// INSTANCE METHODS
impl Node {
    /// Return the sum of this node's counts.
    fn sum(&self) -> usize {
        match self {
            Internal { left, right } =>  { left.sum() + right.sum() }
            Leaf { contents } => contents.precedence()
        }
    }

    pub fn gen_encoding(&self) -> HashMap<u8, BitSequence> {
        let mut encoding: HashMap<u8, BitSequence> = HashMap::new();
        self.visit_node(&mut encoding, BitSequence::new());
        encoding
    }

    // Generate paths to all root nodes.
    // Note: this function takes ownership of the path BitSequence
    // Because each path ought to have its own path.
    fn visit_node(&self, encoding: &mut HashMap<u8, BitSequence>,
        path: BitSequence) {

        match self {
            // If it is an internal node, descend left and right, making this with 0 and 1.
            Internal { left, right } => {
                let mut left_path = path.clone();
                left_path.append_bit(0);
                let mut right_path = path.clone();
                right_path.append_bit(1);

                left.visit_node(encoding, left_path);
                right.visit_node(encoding, right_path);
            }
            // If we've hit a leaf node, add the encoding to the bad boy!
            Leaf { contents } => {
                encoding.insert(contents.byte(), path.clone());
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
            Internal { left, right } => {
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
            Leaf { contents } => {
                f.write_fmt(format_args!("{}: {} ", contents.byte(), contents.precedence()))
            }
        }
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.sum() == other.sum() {
            panic!("Unnormalized data!")
        }
        self.sum().cmp(&other.sum())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}