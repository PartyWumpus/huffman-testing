use bimap::BiMap;
use std::{cell::RefCell, cmp::Reverse, collections::HashMap, rc::Rc};
use bitvec::prelude::*;

#[derive(Debug, Clone)]
enum TreeNode {
    InternalNode {
        left: TreeNodeRef,
        right: TreeNodeRef,
        count: usize,
    },
    LeafNode {
        char: char,
        count: usize,
    },
}
type TreeNodeRef = Rc<RefCell<TreeNode>>;
type HuffmanTable = BiMap<char, usize>;
type BitvecHuffmanTable = BiMap<char, BitVec>;

impl TreeNode {
    fn count(&self) -> usize {
        match self {
            Self::InternalNode { count, .. } | Self::LeafNode { count, .. } => *count,
        }
    }

    fn new_leaf(char: char, count: usize) -> TreeNodeRef {
        Rc::new(RefCell::new(Self::LeafNode { char, count }))
    }

    fn new_internal(left: TreeNodeRef, right: TreeNodeRef) -> TreeNodeRef {
        let mut count = 0;
        count += left.borrow().count();
        count += right.borrow().count();
        Rc::new(RefCell::new(Self::InternalNode { left, right, count }))
    }
}

fn huffman_usize_to_string(num: usize) -> String {
    // format number in binary
    let mut x = format!("{:b}", num);
    // remove trailing 1
    x.remove(0);
    x
}

/// CALCULATING TABLE

fn count_chars(string: &str) -> HashMap<char, usize> {
    let mut chars = HashMap::new();
    for char in string.chars() {
        *chars.entry(char).or_insert(0) += 1;
    }
    chars
}

fn initialize_nodes(counts: HashMap<char, usize>) -> Vec<TreeNodeRef> {
    let mut vec = vec![];
    for (char, count) in counts.iter() {
        vec.push((*char, *count));
    }
    vec.sort_by_key(|(char, count)| (Reverse(*count), *char as u64));
    vec.iter()
        .map(|(char, count)| TreeNode::new_leaf(*char, *count))
        .collect()
}

fn construct_tree(mut nodes: Vec<TreeNodeRef>) -> TreeNodeRef {
    while nodes.len() > 1 {
        let x = TreeNode::new_internal(nodes.pop().unwrap(), nodes.pop().unwrap());
        nodes.push(x);
        nodes.sort_by_key(|node| Reverse(node.borrow().count()));
    }
    nodes.pop().unwrap()
    //Rc::try_unwrap(nodes.pop().expect("array is not empty"))
    //    .expect("there are no other references to the node")
    //    .into_inner()
}

fn bitvec_calculate_encodings(tree: Rc<RefCell<TreeNode>>) -> BitvecHuffmanTable {
    let mut encodings = BiMap::new();
    let mut stack = vec![(tree, BitVec::new())];

    while !stack.is_empty() {
        let (node, index): (Rc<RefCell<TreeNode>>, BitVec) = stack.pop().unwrap();
        match *node.borrow() {
            TreeNode::LeafNode { char, .. } => {
                encodings.insert(char, index.clone());
            }
            TreeNode::InternalNode {
                ref left,
                ref right,
                ..
            } => {
                let mut a = index.clone();
                a.push(true);
                stack.push((Rc::clone(right), a));
                let mut b = index.clone();
                b.push(false);
                stack.push((Rc::clone(left), b));
            }
        };
    }

    encodings
}

fn calculate_encodings(tree: Rc<RefCell<TreeNode>>) -> HuffmanTable {
    let mut encodings = BiMap::new();
    // note the trailing one is used for marking the length
    let mut stack = vec![(tree, 1)];

    while !stack.is_empty() {
        let (node, index): (Rc<RefCell<TreeNode>>, usize) = stack.pop().unwrap();
        match *node.borrow() {
            TreeNode::LeafNode { char, .. } => {
                encodings.insert(char, index);
            }
            TreeNode::InternalNode {
                ref left,
                ref right,
                ..
            } => {
                stack.push((Rc::clone(right), (index << 1) + 1));
                stack.push((Rc::clone(left), (index << 1) + 0));
            }
        };
    }

    encodings
}

fn calculate_huffman_table(str: &str) -> HuffmanTable {
    let counts = count_chars(str);
    let nodes = initialize_nodes(counts);
    let tree = construct_tree(nodes);
    calculate_encodings(tree)
}

fn bitvec_calculate_huffman_table(str: &str) -> BitvecHuffmanTable {
    let counts = count_chars(str);
    let nodes = initialize_nodes(counts);
    let tree = construct_tree(nodes);
    bitvec_calculate_encodings(tree)
}

/// DISPLAYING VISUALLY

fn print_encodings(encodings: &HuffmanTable) -> () {
    print!("{{ ");
    for (char, index) in encodings.iter() {
        let x = huffman_usize_to_string(*index);
        print!(" '{char}' > {x} ")
    }
    print!(" }} \n");
}

fn bitvec_print_encodings(encodings: &BitvecHuffmanTable) -> () {
    print!("{{ ");
    for (char, index) in encodings.iter() {
        print!(" '{char}' > {index} ")
    }
    print!(" }} \n");
}

/// ENCODING DATA USING TABLE

fn bad_encode_usize(str: &str, table: &HuffmanTable) -> usize {
    usize::from_str_radix(
        &str.chars()
            .map(|char| huffman_usize_to_string(*table.get_by_left(&char).unwrap()))
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn usize_huffman_encode(str: &str, table: HuffmanTable) -> usize {
    let mut result = 0;
    for char in str.chars() {
        let index = table.get_by_left(&char).expect("char should be in the table");
        result = result << (usize::BITS - index.leading_zeros() - 1);
        // subtracting the leading one before adding to the result
        result += index - 2_usize.pow(usize::BITS - index.leading_zeros() - 1);
    };
    result
}

fn bitvec_huffman_encode(str: &str, table: BitvecHuffmanTable) -> BitVec {
    let mut vec = BitVec::new();
    for char in str.chars() {
        let index = table.get_by_left(&char).expect("char should be in the table");
        vec.append(&mut index.clone());
    }
    vec
}

fn main() {
    let input = "very long test phrase that would not otherwise fit!";
    //let encodings = calculate_huffman_table(input);
    //print_encodings(&encodings);
    //let x = bad_encode(input, &encodings);
    //let y = usize_huffman_encode(input, encodings);
    //println!("{x:b} == {y:b}, {}", x == y);

    let table = bitvec_calculate_huffman_table(input);
    bitvec_print_encodings(&table);
    let z = bitvec_huffman_encode(input, table);
    println!("{z}");
}
