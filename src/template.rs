use std::collections::HashMap;
use std::ops::Index;

use crate::ast::Node;
use crate::error::Result;
use crate::parser::Parser;

pub enum Context {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Map(HashMap<String, Context>),
    List(Vec<Context>),
}

impl Context {
    fn to_str(&self) -> String {
        match self {
            Context::String(x) => x.to_string(),
            Context::Integer(x) => x.to_string(),
            Context::Float(x) => x.to_string(),
            Context::Bool(x) => x.to_string(),
            _ => panic!("not a scalar"),
        }
    }

    fn get(&self, index: &str) -> Option<&Context> {
        match self {
            Context::Map(x) => x.get(index),
            _ => panic!("not a map"),
        }
    }
}

impl Index<&str> for Context {
    type Output = Context;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            Context::Map(x) => &x[index],
            _ => panic!("not a map"),
        }
    }
}

pub struct Template {
    root: Node,
}

impl Template {
    pub fn compile(text: &str) -> Result<Self> {
        let root = Parser::parse(text)?;
        Ok(Self { root })
    }

    pub fn render(&self, context: Context) -> String {
        Self::render_node(&self.root, &context)
    }

    fn render_node(node: &Node, context: &Context) -> String {
        match node {
            Node::Root(x) => {
                let mut out = String::new();
                for child in &x.children {
                    out.push_str(&Self::render_node(&child, &context));
                }
                out
            }
            Node::Section(x) => match context.get(&x.name) {
                None => String::new(),
                Some(Context::Bool(b)) if *b == true => {
                    let mut out = String::new();
                    for child in &x.children {
                        out.push_str(&Self::render_node(&child, &context));
                    }
                    out.trim().into()
                }
                _ => String::new(),
            },
            Node::Variable(x) => {
                let raw = context.get(&x.name).map_or(String::new(), |v| v.to_str());
                if x.escaped {
                    Self::escape(&raw)
                } else {
                    raw
                }
            }
            Node::Partial(_) => todo!(),
            Node::Text(x) => x.clone(),
        }
    }

    fn escape(input: &str) -> String {
        input.replace("<", "&lt;").replace(">", "&gt;")
    }
}