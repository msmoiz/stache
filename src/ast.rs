use std::ops::Index;

#[derive(Debug, PartialEq)]
pub enum Node {
    Root(Root),
    Section(Section),
    Variable(Variable),
    Partial(String),
    Text(String),
}

impl Node {
    pub fn children(&self) -> &Vec<Node> {
        match self {
            Node::Root(x) => &x.children,
            Node::Section(x) => &x.children,
            _ => panic!("node does not have children"),
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<Node> {
        match self {
            Node::Root(x) => &mut x.children,
            Node::Section(x) => &mut x.children,
            _ => panic!("node does not have children"),
        }
    }

    pub fn push(&mut self, child: Node) {
        match self {
            Node::Root(x) => x.children.push(child),
            Node::Section(x) => x.children.push(child),
            _ => panic!("node does not have children"),
        }
    }
}

impl Index<usize> for Node {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.children()[index]
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Root {
    children: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Variant {
    Direct,
    Inverse,
}

#[derive(Debug, PartialEq)]
pub struct Section {
    name: String,
    variant: Variant,
    children: Vec<Node>,
}

impl Section {
    pub fn new(name: String, variant: Variant) -> Self {
        Self {
            name,
            variant,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    name: String,
    escaped: bool,
}

impl Variable {
    pub fn new(name: String, escaped: bool) -> Self {
        Self { name, escaped }
    }
}
