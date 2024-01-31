use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};
use crate::encoding::bitsequence::BitSequence;
use crate::ordering::bytefreq::ByteFreq;
use crate::tree::node::Node::{Internal, Leaf};

// Author: Will Morris
// A node represents either an internal node, with a left and right child,
// Or a leaf node, with a byte:contents frequency.
// To get the value of a node, descend left and right.
#[derive(Hash, Eq, PartialEq)]
pub enum Node {
    Internal { left: Box<Node>, right: Box<Node> },
    Leaf { contents: ByteFreq },
}

// ****** NODE CONSTRUCTORS ****** //

// HUFFMAN TREE GENERATOR IS ONLY PUBLIC CONSTRUCTOR
pub fn huffman(ordering: &HashMap<u8, u64>) -> Option<Node> {
    // Prepare base heap with all elements sorted by frequency.
    // These are all the leaf nodes.

    // Note that the binary heap is a MAX HEAP, not a min heap.
    // This means that the first items to be removed will be those with the highest precedence.
    // Need to reverse.
    let mut heap = ordering.iter().fold(
        BinaryHeap::new(), | mut heap, (byte, count) | {
            heap.push(leaf(ByteFreq::new(*byte, *count)));
            heap
        });

    // Now prepare internal nodes with children.
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let parent = internal(Box::from(left), Box::from(right));
        heap.push(parent);
    }

    // The last element in the heap is the root node!
    // Note: if no frequencies supplied, this will be none.
    heap.pop()
}

// PRIVATE CONSTRUCTORS USED DURING CREATION OF HUFFMAN TREE
fn leaf(contents: ByteFreq) -> Node { Leaf { contents } }

// Note that internal nodes do consume their children.
fn internal(left: Box<Node>, right: Box<Node>) -> Node { Internal { left, right } }


// PUBLIC INSTANCE METHODS
impl Node {
    // Public interface to generate the BitSequence for the encoding of each byte.
    pub fn gen_encoding(&self) -> HashMap<u8, BitSequence> {
        let mut encoding: HashMap<u8, BitSequence> = HashMap::new();
        // When a leaf is encountered, mark the value to the path traversed.
        let mut visit_fn = | node: &Node, path: &BitSequence | {
            if let Leaf { contents } = node {
                encoding.insert(contents.byte(), path.clone());
            }
        };

        match self {
            Internal { .. } => { self.visit_node(BitSequence::new(), &mut visit_fn) }
            // Edge case: only one node and a path hasn't been formed yet!
            // In this case, encode as 0.
            Leaf { contents } => {
                encoding.insert(contents.byte(), BitSequence::from_bits(&[0]));
            }
        }
        encoding
    }

    // Public interface to generate the BitSequence for the decoding of each byte.
    pub fn gen_decoding(&self) -> HashMap<BitSequence, u8> {
        let mut decoding: HashMap<BitSequence, u8> = HashMap::new();
        // When a leaf node is encountered, mark the path traversed to its value.
        let mut visit_fn = | node: &Node, path: &BitSequence | {
            if let Leaf { contents } = node {
                decoding.insert(path.clone(), contents.byte());
            }
        };

        match self {
            Internal { .. } => { self.visit_node(BitSequence::new(), &mut visit_fn) }
            Leaf { contents } => {
                decoding.insert(BitSequence::from_bits(&[0]), contents.byte());
            }
        }
        decoding
    }

    // Generate paths to all leaf nodes.
    // The visit fns may then do what they will with these paths.
    // This is particularly useful when:
    // 1. You want to traverse with some sort of shared state (i.e. a decoding map)
    // 2. The paths you took to get to nodes are important.
    fn visit_node(&self, path: BitSequence, visit_fn: &mut impl FnMut(&Node, &BitSequence)) {
        match self {
            // If it is an internal node, descend left and right, making this with 0 and 1.
            Internal { left, right } => {
                let mut left_path = path.clone();
                left_path.append_bit(0);
                let mut right_path = path.clone();
                right_path.append_bit(1);

                left.visit_node(left_path, visit_fn);
                right.visit_node(right_path, visit_fn);
            }
            // If we've hit a leaf node, add the encoding to the bad boy!
            Leaf { .. } => { visit_fn(self, &path); }
        }
    }
}


// NODE ATTR ACCESSORS
// useful for comparison
impl Node {
    // These simple visitors are easier to write without using the visitor closure.
    fn freq(&self) -> u64 {
        match self {
            Internal {  left, right, .. } => {
                left.freq() + right.freq()
            }
            Leaf { contents  } => {
                contents.freq()
            }
        }
    }

    // TIEBREAKER
    // What if two nodes have the same frequency?
    // Whichever node contains the minimum byte wins out!
    // For breaking ties in a node, we need the minimum byte.
    fn min_byte(&self) -> u8 {
        match self {
            Internal { left, right } => {
                min(left.min_byte(), right.min_byte())
            }
            Leaf { contents } => {
                contents.byte()
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut visit_fn = | node: &Node, _path: &BitSequence | {
            if let Leaf { contents } = node {
                f.write_fmt(format_args!
                    ("{}: {}", contents.byte(), contents.freq())).unwrap();
            }
        };

        self.visit_node(BitSequence::new(), &mut visit_fn);
        Ok(())
    }
}


// ****** ORD IMPLEMENTATIONS ****** //

impl Ord for Node {
    // NOTE: nodes are done with a MIN HEAP!
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq().cmp(&self.freq())
            .then_with(|| other.min_byte().cmp(&self.min_byte()))
    }
}

// PartialOrd must be implemented or weird things will happen!
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



// ****** TESTS ****** //

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::encoding::bitsequence::BitSequence;
    use crate::tree::node::{huffman};

    // Test that the tree generates an encoding for a single charACTER.
    #[test]
    fn test_single_encoding() {
        let byte = 1;
        let mut freq: HashMap<u8, u64> = HashMap::new();
        freq.insert(byte, 1);

        /*
           EXPECTED ENCODING:
           1: 0
         */

        let mut expected_encoding: HashMap<u8, BitSequence> = HashMap::new();
        expected_encoding.insert(byte, BitSequence::from_bits(&[0]));
        let actual_encoding: HashMap<u8, BitSequence> = huffman(&freq).unwrap().gen_encoding();

        assert_eq!(expected_encoding, actual_encoding);
    }

    // Test that the tree properly generates an encoding
    // NOTE: minor changes in tiebreaking should not ruin this test!
    // Therefore, checking not exact encodings, but rather, encoding lengths.
    #[test]
    fn test_encoding() {
        let mut freq: HashMap<u8, u64> = HashMap::new();
        freq.insert(1, 11);
        freq.insert(0, 4);
        freq.insert(2, 5);
        freq.insert(3, 6);
        freq.insert(4, 1);
        freq.insert(6, 1);
        freq.insert(7, 1);
        freq.insert(5, 2);
        freq.insert(8, 1);
        freq.insert(9, 1);
        let encoding = huffman(&freq).unwrap().gen_encoding();

        assert_eq!(2, encoding.get(&1).unwrap().length());
        assert_eq!(3, encoding.get(&0).unwrap().length());
        assert_eq!(3, encoding.get(&2).unwrap().length());
        assert_eq!(2, encoding.get(&3).unwrap().length());
        assert_eq!(4, encoding.get(&5).unwrap().length());
        assert_eq!(5, encoding.get(&4).unwrap().length());
        assert_eq!(5, encoding.get(&8).unwrap().length());
        assert_eq!(4, encoding.get(&9).unwrap().length());
        assert_eq!(5, encoding.get(&7).unwrap().length());
        assert_eq!(5, encoding.get(&6).unwrap().length());
    }
}