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
type HuffmanTable = BiMap<char, BitVec>;

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

fn calculate_encodings(tree: Rc<RefCell<TreeNode>>) -> HuffmanTable {
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

fn calculate_huffman_tree(str: &str) -> Rc<RefCell<TreeNode>> {
    let counts = count_chars(str);
    let nodes = initialize_nodes(counts);
    construct_tree(nodes)
}

fn calculate_huffman_table(str: &str) -> HuffmanTable {
    let counts = count_chars(str);
    let nodes = initialize_nodes(counts);
    let tree = construct_tree(nodes);
    calculate_encodings(tree)
}

/// DISPLAYING VISUALLY

fn print_encodings(encodings: &HuffmanTable) -> () {
    print!("{{\n");
    for (char, index) in encodings.iter() {
        print!("'{char}' > {index}\n")
    }
    print!("}}\n");
}

/// ENCODING/DECODING DATA

fn huffman_encode(str: &str, table: HuffmanTable) -> BitVec {
    let mut vec = BitVec::new();
    for char in str.chars() {
        let index = table.get_by_left(&char).expect("char should be in the table");
        vec.append(&mut index.clone());
    }
    vec
}

fn huffman_decode(mut bits: BitVec, tree: Rc<RefCell<TreeNode>>) -> String {
    let mut result = String::new();
    let mut node = Rc::clone(&tree);
    bits.reverse(); // reversed because popping is faster
    while !bits.is_empty() {
        let bit = bits.pop().unwrap();
        node = if bit {
            match *node.borrow() {
                TreeNode::InternalNode { ref right, .. } => Rc::clone(right),
                TreeNode::LeafNode { .. } => unreachable!(),
            }
        } else {
            match *node.borrow() {
                TreeNode::InternalNode { ref left, .. } => Rc::clone(left),
                TreeNode::LeafNode { .. } => unreachable!(),
            }
        };

        let x = node.borrow();
        match *x {
            TreeNode::LeafNode { char, .. } => {
                result.push(char);
                drop(x);
                node = Rc::clone(&tree);
            },
            _ => (),
        };
    };
    result
}

// FIXME: strings with only one unique character break, as they are encoded as []

fn main() {
    //let input = "qwertyuiopasdfghjklzxcvbnm1234567890-=[]#';/.,\\";
    //let input = "very long test phrase that would not otherwise fit!";
    let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin orci lorem, lacinia sed finibus id, fermentum non nisi. Aenean ullamcorper, lacus eget porttitor finibus, diam nibh porttitor metus, sed dapibus nisi risus sed nunc. Nam congue dui dolor, vel fringilla felis euismod ut. Cras interdum diam non ornare accumsan. Fusce porta lacus magna, fringilla feugiat turpis lacinia at. Donec leo dui, vulputate vitae magna vitae, consequat commodo odio. Nulla fringilla nisi ligula, sit amet rhoncus magna venenatis non. Quisque libero velit, fermentum a dui at, porttitor tempus diam. Donec fermentum, ex quis cursus lacinia, lectus arcu condimentum dui, vitae pretium tellus massa eget massa. Praesent posuere pretium elementum. Morbi pretium posuere sapien pellentesque tincidunt. Morbi tempor massa nec metus fringilla pharetra. Aliquam pulvinar enim ac ante sagittis imperdiet. Morbi a tristique diam. Nam velit ipsum, pretium at tristique vitae, tincidunt et dolor. Vestibulum sodales ex lacus, vel aliquam risus feugiat facilisis.
Nulla odio ante, accumsan non ultrices non, pulvinar ac enim. Donec maximus sollicitudin commodo. Duis accumsan, tortor a rhoncus consequat, odio ligula pretium metus, dignissim mollis felis nisi sit amet nulla. Phasellus id dignissim erat. Nullam sed lectus aliquet, commodo lacus ac, laoreet sem. Nulla finibus sem at quam lobortis pulvinar. Etiam id cursus lacus, molestie sagittis nibh. Aenean et enim non nulla vehicula rutrum. Etiam ipsum lacus, hendrerit ac dolor sit amet, sagittis aliquam dolor. Nam feugiat dolor urna, quis finibus ipsum cursus aliquam. In id purus ligula. Proin fermentum molestie est et dapibus. Praesent quis elit quis ex euismod molestie porttitor sit amet urna.
Integer non faucibus urna, nec tempor justo. Etiam aliquam dui diam, quis malesuada nibh lobortis non. Aliquam eget aliquet turpis, placerat gravida diam. Sed placerat accumsan feugiat. Sed ut pellentesque lorem. Suspendisse sit amet ligula metus. Aenean non mollis sem. Nunc vehicula, mauris sed pellentesque pulvinar, enim nisi iaculis dui, a eleifend nulla risus vel metus. Praesent at dui viverra metus imperdiet venenatis sit amet id tellus. Cras lacinia vel neque id condimentum. Sed et dui tortor. Interdum et malesuada fames ac ante ipsum primis in faucibus. Curabitur nec hendrerit nunc. Nulla blandit purus odio, in faucibus enim feugiat ac. Morbi auctor eleifend tellus, ut ultrices lorem mollis ac.
Phasellus lacus lacus, laoreet ac orci a, dictum porta velit. In hac habitasse platea dictumst. Quisque ultricies ante at porttitor sagittis. Nunc aliquam faucibus urna eget aliquam. Quisque non nibh velit. Pellentesque rutrum blandit sem, in efficitur magna varius at. Etiam sollicitudin pretium venenatis. Duis accumsan tellus ex, aliquam sollicitudin elit lacinia dapibus.
Morbi vulputate hendrerit lobortis. Curabitur suscipit mauris ex. Ut mollis augue ut augue blandit, eu aliquet velit malesuada. Integer eu suscipit nunc, ornare facilisis lorem. Pellentesque vitae orci dapibus, pharetra elit id, sagittis elit. Vestibulum facilisis, odio sit amet commodo viverra, mauris orci porttitor est, vel rutrum ipsum dui sed tortor. Ut in quam cursus, pulvinar sapien non, fermentum diam. Nulla cursus sagittis sapien, eu tincidunt neque interdum sed. Nulla nisl velit, aliquam vitae ante ut, vehicula ornare nisl. Nullam laoreet eros in erat gravida, at maximus diam iaculis.
";
    println!("{input}");

    let tree = calculate_huffman_tree(input);
    let table = calculate_encodings(tree.clone());
    print_encodings(&table);
    let x = huffman_encode(input, table.clone());
    println!("{x}");
    let y = huffman_decode(x.clone(), tree);
    assert!(y == input);

    let char_size = std::mem::size_of::<char>() * 8;
    let original_size = input.len() * char_size;
    // assumes optimal packing of huffman table
    let huffman_size = 
        x.len() + table.into_iter().map(|(_, bits)| char_size + bits.len()).sum::<usize>();
    println!("before: {original_size}, after: {huffman_size}, ratio: {:.2}x original size", (huffman_size as f64) / (original_size as f64))
}
