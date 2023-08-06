#![allow(dead_code)]

use std::slice::Iter;

use crate::{
    ast::{Node, Partial, Root, Section, Variable, Variant},
    error::{Error, Result},
    lexer::{Lexer, Token},
};

pub struct Parser<'t> {
    lexer: Lexer<'t>,
}

impl<'t> Parser<'t> {
    fn new(text: &'t str) -> Self {
        Self {
            lexer: Lexer::new(text),
        }
    }

    pub fn parse(text: &'t str) -> Result<Node> {
        let mut parser = Parser::new(text);
        parser.root()
    }

    fn root(&mut self) -> Result<Node<'t>> {
        let tokens = self.lexer.tokens()?;
        let mut token_it = tokens.iter();
        let mut root = Node::Root(Root::default());
        while let Some(token) = token_it.next() {
            let node = match token {
                Token::Comment => continue,
                Token::SetDelim(..) => continue,
                Token::Text(x) => Node::Text(x),
                Token::Whitespace(x) => Node::Text(x),
                Token::Newline(x) => Node::Text(x),
                Token::Variable(name, esc) => Node::Variable(Variable::new(name, *esc)),
                Token::Partial(name, indent) => Node::Partial(Partial::new(name, indent.clone())),
                Token::SectionStart(name, variant) => {
                    Self::section(&name, &variant, &mut token_it)?
                }
                Token::SectionEnd(..) => return Err(Error::Parse),
            };
            root.push(node);
        }
        Ok(root)
    }

    fn section(
        name: &'t str,
        variant: &Variant,
        token_it: &mut Iter<Token<'t>>,
    ) -> Result<Node<'t>> {
        let mut section = Node::Section(Section::new(name.into(), *variant));
        while let Some(token) = token_it.next() {
            let node = match token {
                Token::Comment => continue,
                Token::SetDelim(..) => continue,
                Token::Text(x) => Node::Text(x),
                Token::Whitespace(x) => Node::Text(x),
                Token::Newline(x) => Node::Text(x),
                Token::Variable(name, esc) => Node::Variable(Variable::new(name, *esc)),
                Token::Partial(name, indent) => Node::Partial(Partial::new(name, indent.clone())),
                Token::SectionStart(name, variant) => Self::section(&name, &variant, token_it)?,
                Token::SectionEnd(end_name) => match end_name == &name {
                    true => break,
                    false => return Err(Error::Parse),
                },
            };
            section.push(node);
        }
        Ok(section)
    }
}
