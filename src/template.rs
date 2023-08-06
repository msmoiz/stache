use std::collections::HashMap;

use crate::ast::{Node, Partial, Root, Section, Variable, Variant};
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
    fn to_string(&self) -> String {
        match self {
            Context::String(x) => x.to_string(),
            Context::Integer(x) => x.to_string(),
            Context::Float(x) => x.to_string(),
            Context::Bool(x) => x.to_string(),
            Context::Null => "".to_string(),
            _ => panic!("not a scalar"),
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Context::String(_) | Context::Integer(_) | Context::Float(_) => true,
            Context::Bool(x) => *x,
            Context::Null => false,
            Context::Map(_) => true,
            Context::List(x) => x.len() > 0,
        }
    }

    fn get(&self, index: &str) -> Option<&Context> {
        match self {
            Context::Map(x) => x.get(index),
            _ => None,
        }
    }
}

pub type Partials = HashMap<String, String>;

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

    fn find(&self, name: &str) -> Option<&Context> {
        if name == "." {
            return self.stack.last().copied();
        }

        let segments: Vec<&str> = name.split(".").collect();

        let mut out = None;
        for context in self.stack.iter().rev() {
            if let Some(context) = context.get(segments[0]) {
                out = Some(context);
                break;
            }
        }

        for segment in &segments[1..] {
            match out {
                None => break,
                Some(context @ Context::Map(_)) => out = context.get(&segment),
                _ => out = None,
            }
        }

        out
    }
}

pub struct Template<'t> {
    root: Node<'t>,
}

impl<'t> Template<'t> {
    pub fn compile(text: &'t str) -> Result<Self> {
        let root = Parser::parse(text)?;
        Ok(Self { root })
    }

    pub fn render(&self, context: Context) -> String {
        Self::render_node(&self.root, ContextResolver::new(&context), &Partials::new())
    }

    pub fn render_with_partials(&self, context: Context, partials: Partials) -> String {
        Self::render_node(&self.root, ContextResolver::new(&context), &partials)
    }

    fn render_node(node: &Node, resolver: ContextResolver, partials: &Partials) -> String {
        match node {
            Node::Root(root) => Self::render_root(root, resolver, partials),
            Node::Section(section) => Self::render_section(section, resolver, partials),
            Node::Variable(variable) => Self::render_variable(variable, resolver),
            Node::Partial(partial) => Self::render_partial(partial, resolver, partials),
            Node::Text(text) => Self::render_text(text),
        }
    }

    fn render_root(root: &Root, resolver: ContextResolver, partials: &Partials) -> String {
        root.children
            .iter()
            .map(|child| Self::render_node(&child, resolver.clone(), partials))
            .collect::<Vec<String>>()
            .join("")
    }

    fn render_section(section: &Section, resolver: ContextResolver, partials: &Partials) -> String {
        let context = resolver.find(section.name);
        match (section.variant, context) {
            (Variant::Direct, Some(Context::List(list))) if !list.is_empty() => {
                let mut out = String::new();
                for context in list {
                    for child in &section.children {
                        out.push_str(&Self::render_node(&child, resolver.push(context), partials));
                    }
                }
                out
            }
            (Variant::Direct, Some(context)) if context.is_truthy() => section
                .children
                .iter()
                .map(|child| Self::render_node(&child, resolver.push(context), partials))
                .collect::<Vec<String>>()
                .join(""),
            (Variant::Inverse, Some(context)) if !context.is_truthy() => section
                .children
                .iter()
                .map(|child| Self::render_node(&child, resolver.clone(), partials))
                .collect::<Vec<String>>()
                .join(""),
            (Variant::Inverse, None) => section
                .children
                .iter()
                .map(|child| Self::render_node(&child, resolver.clone(), partials))
                .collect::<Vec<String>>()
                .join(""),
            _ => String::new(),
        }
    }

    fn render_variable(variable: &Variable, resolver: ContextResolver) -> String {
        let raw = resolver
            .find(variable.name)
            .map_or(String::new(), |context| context.to_string());
        if variable.escaped {
            Self::escape(&raw)
        } else {
            raw
        }
    }

    fn render_partial(partial: &Partial, resolver: ContextResolver, partials: &Partials) -> String {
        let Partial { name, indent } = partial;
        match partials.get(*name) {
            None => String::new(),
            Some(partial) => match Template::compile(&Self::indent(partial, &indent)) {
                Err(_) => String::new(),
                Ok(template) => Self::render_node(&template.root, resolver, partials),
            },
        }
    }

    fn render_text(text: &str) -> String {
        text.into()
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

    fn indent(partial: &str, indent: &str) -> String {
        Self::lines(partial)
            .iter()
            .map(|line| indent.to_owned() + line)
            .collect::<Vec<String>>()
            .join("")
    }

    fn lines(input: &str) -> Vec<&str> {
        let mut lines = Vec::new();
        let mut line_start = 0;
        let mut line_len = 0;
        for c in input.chars() {
            line_len += 1;
            if c == '\n' {
                lines.push(&input[line_start..line_start + line_len]);
                line_start = line_start + line_len;
                line_len = 0;
            }
        }
        if line_len > 0 {
            lines.push(&input[line_start..line_start + line_len]);
        }
        lines
    }
}
