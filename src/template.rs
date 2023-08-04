use std::collections::HashMap;
use std::ops::Index;

use crate::ast::{Node, Variant};
use crate::error::Result;
use crate::parser::Parser;

pub enum Context {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Null,
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
            Context::Null => "".to_string(),
            _ => panic!("not a scalar"),
        }
    }

    fn get(&self, index: &str) -> Option<&Context> {
        if index == "." {
            return Some(self);
        }
        let mut context = Some(self);
        for index in index.split(".") {
            context = match context {
                Some(Context::Map(x)) => x.get(index),
                _ => None,
            };
        }
        context
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

#[derive(Clone)]
pub struct ContextResolver<'a> {
    stack: Vec<&'a Context>,
}

impl<'a> ContextResolver<'a> {
    fn new(base: &'a Context) -> Self {
        Self { stack: vec![base] }
    }

    fn push(&self, context: &'a Context) -> Self {
        let mut clone = self.clone();
        clone.stack.push(context);
        clone
    }

    fn get(&self, name: &str) -> Option<&Context> {
        let segments: Vec<&str> = name.split(".").collect();
        let first_name = if name == "." { "." } else { segments[0] };
        let mut first_context = None;
        for context in self.stack.iter().rev() {
            if let Some(val) = context.get(first_name) {
                first_context = Some(val);
                break;
            }
        }
        if segments.len() - 1 > 0 && name != "." {
            if let Some(first_context) = first_context {
                return first_context.get(&segments[1..].join("."));
            }
        }
        first_context
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
        Self::render_node(&self.root, ContextResolver::new(&context))
    }

    fn render_node(node: &Node, resolver: ContextResolver) -> String {
        match node {
            Node::Root(x) => {
                let mut out = String::new();
                for child in &x.children {
                    out.push_str(&Self::render_node(&child, resolver.clone()));
                }
                out
            }
            Node::Section(x) => match x.variant {
                Variant::Direct => match resolver.get(&x.name) {
                    None => String::new(),
                    Some(c @ Context::Integer(_)) => {
                        let mut out = String::new();
                        for child in &x.children {
                            out.push_str(&Self::render_node(&child, resolver.push(c)));
                        }
                        out
                    }
                    Some(c @ Context::String(_)) => {
                        let mut out = String::new();
                        for child in &x.children {
                            out.push_str(&Self::render_node(&child, resolver.push(c)));
                        }
                        out
                    }
                    Some(c @ Context::Bool(b)) if *b == true => {
                        let mut out = String::new();
                        for child in &x.children {
                            out.push_str(&Self::render_node(&child, resolver.push(c)));
                        }
                        out
                    }
                    Some(c @ Context::Map(_)) => {
                        let mut out = String::new();
                        for child in &x.children {
                            out.push_str(&Self::render_node(&child, resolver.push(c)));
                        }
                        out
                    }
                    Some(Context::List(list)) if !list.is_empty() => {
                        let mut out = String::new();
                        for c in list {
                            for child in &x.children {
                                out.push_str(&Self::render_node(&child, resolver.push(c)));
                            }
                        }
                        out
                    }
                    _ => String::new(),
                },
                Variant::Inverse => match resolver.get(&x.name) {
                    None => {
                        let mut out = String::new();
                        for child in &x.children {
                            out.push_str(&Self::render_node(&child, resolver.clone()));
                        }
                        out
                    }
                    Some(Context::Bool(b)) if *b == false => {
                        let mut out = String::new();
                        for child in &x.children {
                            out.push_str(&Self::render_node(&child, resolver.clone()));
                        }
                        out
                    }
                    Some(Context::Null) => {
                        let mut out = String::new();
                        for child in &x.children {
                            out.push_str(&Self::render_node(&child, resolver.clone()));
                        }
                        out
                    }
                    Some(Context::List(list)) if list.is_empty() => {
                        let mut out = String::new();
                        for child in &x.children {
                            out.push_str(&Self::render_node(&child, resolver.clone()));
                        }
                        out
                    }
                    _ => String::new(),
                },
            },
            Node::Variable(x) => {
                let raw = resolver.get(&x.name).map_or(String::new(), |v| v.to_str());
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
        const ESCAPES: [(&str, &str); 5] = [
            ("&", "&amp;"),
            (">", "&gt;"),
            ("<", "&lt;"),
            ("\"", "&quot;"),
            ("'", "&#39;"),
        ];
        let mut out = String::from(input);
        for (from, to) in ESCAPES {
            out = out.replace(from, to);
        }
        out
    }
}
