use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{cell::RefCell, rc::Rc};
use std::collections::{HashMap, BTreeMap};
use std::path::PathBuf;

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default)]
struct Node {
    size: usize,
    children: BTreeMap<PathBuf, NodeHandle>,
    parent: Option<NodeHandle>,
}

fn main() {
    let file = File::open("./src/input.txt").expect("Couldn't open File.");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().into_iter().map(|l| l.unwrap()).collect();

    /*
        $ cd /      -> Go to home dir
        $ cd name   -> go into dir <name>
        $ cd ..     -> go out of current dir
        $ ls        -> list directories and files
        1234 name   -> size, filename
        dir name    -> contains directory <name>
    */

    println!("{:?}", lines);

    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();
}
