use std::ops::Index;

#[derive(Debug, PartialEq)]
pub enum Node<'t> {
    Root(Root<'t>),
    Section(Section<'t>),
    Variable(Variable<'t>),
    Partial(Partial<'t>),
    Text(&'t str),
}

impl<'t> Node<'t> {
    pub fn children(&self) -> &Vec<Node<'t>> {
        match self {
            Node::Root(x) => &x.children,
            Node::Section(x) => &x.children,
            _ => panic!("node does not have children"),
        }
    }

    pub fn push(&mut self, child: Node<'t>) {
        match self {
            Node::Root(x) => x.children.push(child),
            Node::Section(x) => x.children.push(child),
            _ => panic!("node does not have children"),
        }
    }
}

impl<'t> Index<usize> for Node<'t> {
    type Output = Node<'t>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.children()[index]
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Root<'t> {
    pub children: Vec<Node<'t>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Variant {
    Direct,
    Inverse,
}

#[derive(Debug, PartialEq)]
pub struct Section<'t> {
    pub name: &'t str,
    pub variant: Variant,
    pub children: Vec<Node<'t>>,
}

impl<'t> Section<'t> {
    pub fn new(name: &'t str, variant: Variant) -> Self {
        Self {
            name,
            variant,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Variable<'t> {
    pub name: &'t str,
    pub escaped: bool,
}

impl<'t> Variable<'t> {
    pub fn new(name: &'t str, escaped: bool) -> Self {
        Self { name, escaped }
    }
}

#[derive(Debug, PartialEq)]
pub struct Partial<'t> {
    pub name: &'t str,
    pub indent: String,
}

impl<'t> Partial<'t> {
    pub fn new(name: &'t str, indent: String) -> Self {
        Self { name, indent }
    }
}
