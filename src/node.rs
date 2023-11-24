use crate::freq;

// A node represents a 
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    contents: Option<freq::FreqCount>,
}
