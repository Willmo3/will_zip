# Will_Zip

## About

### What is it?
WillZip is a command line utility that compresses files using the Huffman coding algorithm. It's like Gzip... just a lot worse. (Hey, those folks are professionals!)

I wrote it in Rust because I wanted to learn the language better, and also because I really appreciate Rust's crate system for dependency management. Anyone who has ever had to delve into the depths of Gradle will surely agree with me here!

### Why Does it Exist?
I undertook this project over a gripe. 

In my Data Structures and Algorithms course, we built a Huffman compressor in Java. To save time, the instructor provided us built-in code for saving the actual files, so that pretty much all we had to do was build the actual Huffman tree.

But the instructor code serialized the objects using Java's default serialization scheme. So the file format:

 1. Could only be deserialized by Java code
 2. Was wastefully oversized. Encoded files would even have Java keywords strewn about them!

In my anger, WillZip was born!

## Usage

    wz
     -u (usage)
     -i (input file)
     -o (output file)
     -z (compress input file, mutually exclusive with -x)
     -x (extract input file, mutually exclusive with -z)

## Design Choices
WillZip is a work in progress!

### Serialization
Will_Zip is concerned with *size*. We want small files! As a result, using a tool like Serde to spew out Json wasn't an option. A custom serialization scheme was needed!

To solve this issue, I created the `bytestream` trait, representing anything that can be constructed into and destructed from a byte array.

    pub trait ByteStream {  
	    type Data;
    
	    fn from_stream(bytes: &[u8]) -> Self::Data;
	    fn to_stream(self) -> Vec<u8>;
    }

A file, then, is merely a collection of different objects that can be turned into ByteStreams, bounded by an 8-byte integer representing the object's size. (More about integer size later!)

### Encoding

 - Frequency table length
 - Frequency table, mapping each byte in original file to its frequency in that file.
 - Content
 - A stream of bytes that can be constructed into a list of bits, representing the encoded original file.

### Sizing
An immediate issue with my compressor is the size of the values in the frequency table. 

It would be nice to just encode a nice little two-byte integer to represent the frequency of each object. But what happens when the file is so large that an individual byte's frequency overflows this bound?

#### The Dream of Normalization
My original idea was to "normalize" the data: instead of storing each character's absolute frequency, I would instead store an ordinal value representing its relative frequency.

 - A: 0 (since it's the most common)
 - C: 3 (least common)
 - B: 2
 - u: 1

Think about "0th", "1st", and so on place. 
The beauty of this is that since there are only 256 unique bytes, there are only 256 positions to store. Instead of storing a byte's number of occurrences as an 8-byte integer, we can store a single byte representing its ordinal frequency!

This would slash encoding size, particularly in small files, where metadata like the frequency table often bloats the encoded file beyond its original size!

#### The Dream.. Or the Nightmare?
But it turned out I was missing something very important about Huffman encoding. Huffman relies on more than knowing just the *order* of the elements. It needs to know their full frequency.

Think of the frequency of a byte. Suppose we've got a file, and there are 20 `a's` in it.

    a: 20

Now suppose the only other bytes in the file are 5 `b's` and 5 `c's`.

    a: 20
    b: 5
    c: 5
   
 If we "normalized" this as I wanted to, we would end up with something like this (breaking ties with minimum byte value)
 

    a: 0
    b: 1
    c: 2
   
 What we've *lost* here is just how much *more* frequent `a` is than the rest of them. Storing the frequency of each byte implicitly stores its frequency in the entire file, not just whether it occured more often than anything else.

And this is how Huffman breaks ties!
Suppose we follow the normalization scheme, popping the least frequent byte first (i.e. c and b). Now, suppose we've used a max heap for this problem, so that things with the highest "placement" value are considered the least frequent and are popped first.

Then on our first iteration, we'll end up with an internal node of value $n$ and value $n-1$, where $n$ is the number of unique bytes in the file.

The internal node will thus have value $2n - 1$. When we pop the next node, it will *have* to have the value $n-2$, because we normalized the data. And so a new internal node will be created, with the current internal node as its left child and the new leaf node as its right.

Now the internal node has weight $3n - 3$. This is strictly larger than anything else in our normalized tree. As this process repeats, we'll have the internal node as the left child again, and the right node will be another leaf.

So we're guaranteed to have a completely lopsided tree! To reach each leaf, we will have to descend leftward a ton of times, meaning there are a ton of 0s in our encoding, and only go right once. 

The encodings will quickly balloon in size because we've effectively guaranteed a pathological case to our encoder!

#### The Future of Sizing
So it's curtains on normalization. But encoding 9-byte `u8 -> u64` mappings for each byte in the original file hardly seems like a compression style to write home about! For small files, we're going to end up with larger encoded versions, which is exactly what we set out to avoid!

A key insight is that if the file is small, we won't actually need 8 bytes to encode each frequency. My plan for the future is thus to find the minimum number of bytes needed to encode the largest frequency, and encode that metadata into the frequency table deserializer. That way, we don't waste any more space than we need to!

### Visiting
So we're visiting a tree. Do we or don't we use the infamous visitor pattern?

Well... yes and no.
The visitor pattern emulates many of the tools we get for free in Rust. Observe this code:

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
   
   Enums + match statements give us an easy way to traverse heterogenous data structures, which is ultimately the point of the visitor pattern. In this case, traversing the tree to gain frequency is easy.

But what happens when we want to traverse the tree to get the paths to each struct?

 - Start at the root node
 - Descend into the left and right subtrees
 - If we reach a leaf node, store the path it took to get here.
 - Otherwise, repeat.

There's a lot of boilerplate. Each traversal needs to store the path it took to get there from the root. And if you're "storing" the path it took to get here... where are you storing it?

In our case, encoding means tying each byte to the sequence of bits it should be represented by, or its position in the tree.
But decoding means tying each stream of bits to the byte they represent!

Encoding and decoding are very similar, but just different enough that we can't put them in the same function. We need a way to abstract these walks so that any function that needs to get paths can access them.

Enter `visit_node`!

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

Now, the encoding/decoding specific operations on the leaf node are contained within "visit_fn". This is far more extensible than copying and pasting a bunch of the same code!

## Future Steps

 - Make BitSequence use  an interator
 - Allow piping from stdin/out
 - Resize serialized frequency tree to minimum required size
