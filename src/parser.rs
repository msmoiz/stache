#![allow(dead_code)]

use std::slice::Iter;

use crate::{
    ast::{Node, Partial, Root, Section, Variable, Variant},
    error::{Error, Result},
    lexer::{Lexer, Token},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            lexer: Lexer::new(text),
        }
    }

    pub fn parse(text: &str) -> Result<Node> {
        let mut parser = Parser::new(text);
        parser.root()
    }

    fn root(&mut self) -> Result<Node> {
        let tokens = self.lexer.tokens()?;
        let mut token_it = tokens.iter();
        let mut root = Node::Root(Root::default());
        while let Some(token) = token_it.next() {
            match token {
                Token::Comment => continue,
                Token::SetDelim(..) => continue,
                Token::Text(content) => root.push(Node::Text(content.clone())),
                Token::Whitespace(content) => root.push(Node::Text(content.clone())),
                Token::Newline(content) => root.push(Node::Text(content.clone())),
                Token::Variable(name, esc) => {
                    root.push(Node::Variable(Variable::new(name.clone(), *esc)))
                }
                Token::Partial(name, indent) => {
                    root.push(Node::Partial(Partial::new(name.clone(), indent.clone())))
                }
                Token::SectionStart(name, variant) => {
                    root.push(self.section(&name, &variant, &mut token_it)?)
                }
                Token::SectionEnd(..) => return Err(Error::Parse),
            }
        }
        Ok(root)
    }

    fn section(
        &mut self,
        name: &str,
        variant: &Variant,
        token_it: &mut Iter<Token>,
    ) -> Result<Node> {
        let mut section = Node::Section(Section::new(name.into(), *variant));
        while let Some(token) = token_it.next() {
            match token {
                Token::Comment => continue,
                Token::SetDelim(..) => continue,
                Token::Text(content) => section.push(Node::Text(content.clone())),
                Token::Whitespace(content) => section.push(Node::Text(content.clone())),
                Token::Newline(content) => section.push(Node::Text(content.clone())),
                Token::Variable(name, esc) => {
                    section.push(Node::Variable(Variable::new(name.clone(), *esc)))
                }
                Token::Partial(name, indent) => {
                    section.push(Node::Partial(Partial::new(name.clone(), indent.clone())))
                }
                Token::SectionStart(name, variant) => {
                    section.push(self.section(&name, &variant, token_it)?)
                }
                Token::SectionEnd(end_name) => match end_name == name {
                    true => break,
                    false => return Err(Error::Parse),
                },
            }
        }
        Ok(section)
    }
}
