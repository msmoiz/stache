use std::collections::HashMap;

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
    pub(crate) fn to_string(&self) -> String {
        match self {
            Context::String(x) => x.to_string(),
            Context::Integer(x) => x.to_string(),
            Context::Float(x) => x.to_string(),
            Context::Bool(x) => x.to_string(),
            Context::Null => "".to_string(),
            _ => panic!("not a scalar"),
        }
    }

    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Context::String(_) | Context::Integer(_) | Context::Float(_) => true,
            Context::Bool(x) => *x,
            Context::Null => false,
            Context::Map(_) => true,
            Context::List(x) => x.len() > 0,
        }
    }

    pub(crate) fn get(&self, index: &str) -> Option<&Context> {
        match self {
            Context::Map(x) => x.get(index),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct MapBuilder {
    context: HashMap<String, Context>,
}

impl MapBuilder {
    pub fn new() -> Self {
        MapBuilder::default()
    }

    pub fn str<'a>(mut self, key: &str, value: &str) -> Self {
        self.context
            .insert(key.into(), Context::String(value.into()));
        self
    }

    pub fn int<'a>(mut self, key: &str, value: i64) -> Self {
        self.context.insert(key.into(), Context::Integer(value));
        self
    }

    pub fn float<'a>(mut self, key: &str, value: f64) -> Self {
        self.context.insert(key.into(), Context::Float(value));
        self
    }

    pub fn bool<'a>(mut self, key: &str, value: bool) -> Self {
        self.context.insert(key.into(), Context::Bool(value));
        self
    }

    pub fn list<'a>(mut self, key: &str, value: Vec<Context>) -> Self {
        self.context.insert(key.into(), Context::List(value));
        self
    }

    pub fn map<'a>(mut self, key: &str, value: HashMap<String, Context>) -> Self {
        self.context.insert(key.into(), Context::Map(value));
        self
    }

    pub fn build(self) -> Context {
        Context::Map(self.context)
    }
}

#[derive(Default)]
pub struct VecBuilder {
    context: Vec<Context>,
}

impl VecBuilder {
    pub fn new() -> Self {
        VecBuilder::default()
    }

    pub fn str<'a>(mut self, value: &str) -> Self {
        self.context.push(Context::String(value.into()));
        self
    }

    pub fn int<'a>(mut self, value: i64) -> Self {
        self.context.push(Context::Integer(value));
        self
    }

    pub fn float<'a>(mut self, value: f64) -> Self {
        self.context.push(Context::Float(value));
        self
    }

    pub fn bool<'a>(mut self, value: bool) -> Self {
        self.context.push(Context::Bool(value));
        self
    }

    pub fn list<'a>(mut self, value: Vec<Context>) -> Self {
        self.context.push(Context::List(value));
        self
    }

    pub fn map<'a>(mut self, value: HashMap<String, Context>) -> Self {
        self.context.push(Context::Map(value));
        self
    }

    pub fn build(self) -> Context {
        Context::List(self.context)
    }
}
