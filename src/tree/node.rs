use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};
use crate::encoding::bitsequence::BitSequence;
use crate::ordering::freq::ByteFreq;
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
pub fn huffman(ordering: &HashMap<u8, usize>) -> Option<Node> {
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
        match self {
            Internal { .. } => {
                self.visit_node(&mut encoding, BitSequence::new());
            }
            // Edge case: only one node and a path hasn't been formed yet!
            // In this case, encode as 0.
            Leaf { contents } => {
                encoding.insert(contents.byte(), BitSequence::from(&[0]));
            }
        }
        encoding
    }

    // Generate paths to all leaf nodes.
    // Note: this function takes ownership of the path BitSequence
    // Because each leaf node ought to have its own path
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


// NODE ATTR ACCESSORS
// useful for comparison
impl Node {
    fn freq(&self) -> usize {
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
        match self {
            Internal { left, right } => {
                let left = left.fmt(f);
                if left.is_err() {
                    return left
                }
                let right = right.fmt(f);
                if right.is_err() {
                    return right
                }
                Ok(())
            }
            Leaf { contents } => {
                f.write_fmt(format_args!("{}: {} ", contents.byte(), contents.freq()))
            }
        }
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
    use crate::ordering::freq::gen_frequency;
    use crate::tree::node::{huffman};

    // Test that the tree generates an encoding for a single charACTER.
    #[test]
    fn test_single_encoding() {
        let byte = 1;
        let mut freq: HashMap<u8, usize> = HashMap::new();
        freq.insert(byte, 1);

        /*
           EXPECTED ENCODING:
           1: 0
         */

        let mut expected_encoding: HashMap<u8, BitSequence> = HashMap::new();
        expected_encoding.insert(byte, BitSequence::from(&[0]));
        let actual_encoding: HashMap<u8, BitSequence> = huffman(&freq).unwrap().gen_encoding();

        assert_eq!(expected_encoding, actual_encoding);
    }

    // Test that the tree properly generates an encoding
    #[test]
    fn test_encoding() {
        // TODO: redo test.
        // Need to account for relative ordering.

        /*
           STRING: 1111100022334

           EXPECTED ENCODING:
           1: 1
           0: 00
           3: 011
           2: 0100
           4: 0101
         */

        let mut expected_encoding: HashMap<u8, BitSequence> = HashMap::new();
        expected_encoding.insert(1, BitSequence::from(&[1]));
        expected_encoding.insert(0, BitSequence::from(&[0, 0]));
        expected_encoding.insert(3, BitSequence::from(&[0, 1, 1]));
        expected_encoding.insert(2, BitSequence::from(&[0, 1, 0, 0]));
        expected_encoding.insert(4, BitSequence::from(&[0, 1, 0, 1]));


        let bytes: Vec<u8> = "11111111110000ASDABV2233433223123".bytes().collect();
        let freq = gen_frequency(&bytes);

        let actual_encoding = huffman(&freq).unwrap().gen_encoding();
        assert_eq!(expected_encoding, actual_encoding);
    }
}