#![allow(dead_code)]

use crate::{
    ast::{Node, Root, Section, Variable, Variant},
    error::{Error, Result},
};

#[derive(Debug, PartialEq)]
enum Token {
    Text(usize, usize),
    Variable(usize, usize, bool),
    SectionStart(usize, usize),
    InvertSectionStart(usize, usize),
    SectionEnd(usize, usize),
    Partial(usize, usize),
    SetDelim(String, String),
    Comment,
}

pub struct Parser<'a> {
    text: &'a str,
    pos: usize,
    open_delim: String,
    close_delim: String,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            pos: 0,
            open_delim: String::from("{{"),
            close_delim: String::from("}}"),
        }
    }

    pub fn parse(text: &str) -> Result<Node> {
        let mut parser = Parser::new(text);
        parser.root()
    }

    fn root(&mut self) -> Result<Node> {
        let mut root = Node::Root(Root::default());
        while let Some(token) = self.next_token()? {
            match token {
                Token::Comment => continue,
                Token::Text(pos, len) => root.push(Node::Text(self.text[pos..pos + len].into())),
                Token::Variable(pos, len, escaped) => root.push(Node::Variable(Variable::new(
                    self.text[pos..pos + len].into(),
                    escaped,
                ))),
                Token::Partial(pos, len) => {
                    root.push(Node::Partial(self.text[pos..pos + len].into()))
                }
                Token::SectionStart(pos, len) => {
                    let name = &self.text[pos..pos + len];
                    root.push(self.section(name, Variant::Direct)?);
                }
                Token::InvertSectionStart(pos, len) => {
                    let name = &self.text[pos..pos + len];
                    root.push(self.section(name, Variant::Inverse)?);
                }
                Token::SectionEnd(..) => return Err(Error::Parse),
                Token::SetDelim(open, close) => {
                    self.open_delim = open;
                    self.close_delim = close;
                }
            }
        }
        Ok(root)
    }

    fn section(&mut self, name: &str, variant: Variant) -> Result<Node> {
        let mut section = Node::Section(Section::new(name.into(), variant));
        while let Some(token) = self.next_token()? {
            match token {
                Token::Comment => continue,
                Token::Text(pos, len) => section.push(Node::Text(self.text[pos..pos + len].into())),
                Token::Variable(pos, len, escaped) => section.push(Node::Variable(Variable::new(
                    self.text[pos..pos + len].into(),
                    escaped,
                ))),
                Token::Partial(pos, len) => {
                    section.push(Node::Partial(self.text[pos..pos + len].into()))
                }
                Token::SectionStart(pos, len) => {
                    let name = &self.text[pos..pos + len];
                    section.push(self.section(name, Variant::Direct)?);
                }
                Token::InvertSectionStart(pos, len) => {
                    let name = &self.text[pos..pos + len];
                    section.push(self.section(name, Variant::Inverse)?);
                }
                Token::SectionEnd(pos, len) => {
                    let end_name = &self.text[pos..pos + len];
                    if end_name == name {
                        break;
                    } else {
                        return Err(Error::Parse);
                    }
                }
                Token::SetDelim(open, close) => {
                    self.open_delim = open;
                    self.close_delim = close;
                }
            }
        }
        Ok(section)
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        if self.pos == self.text.len() {
            return Ok(None);
        }

        if let Some((token, len)) = self.scan_set_delim()? {
            self.pos += len;
            return Ok(Some(token));
        }

        if let Some((token, len)) = self.scan_triple_unescape()? {
            self.pos += len;
            return Ok(Some(token));
        }

        if let Some((token, len)) = self.scan_tag()? {
            self.pos += len;
            return Ok(Some(token));
        }

        let (token, len) = self.scan_text();
        self.pos += len;
        Ok(Some(token))
    }

    fn remainder(&self) -> &str {
        &self.text[self.pos..]
    }

    fn scan_set_delim(&self) -> Result<Option<(Token, usize)>> {
        let Some(remainder) = self.remainder().strip_prefix(&format!("{}=", &self.open_delim)) else {
            return Ok(None);
        };
        let Some(content_len) = remainder.find(&format!("={}", &self.close_delim)) else {
            return Err(Error::Parse);
        };
        let mut new_delims = remainder[..content_len].trim().split_whitespace();
        let (Some(open_delim), Some(close_delim)) = (new_delims.next(), new_delims.next()) else {
            return Err(Error::Parse);
        };
        Ok(Some((
            Token::SetDelim(open_delim.into(), close_delim.into()),
            3 + content_len + 3,
        )))
    }

    fn scan_triple_unescape(&self) -> Result<Option<(Token, usize)>> {
        let Some(remainder) = self.remainder().strip_prefix(&format!("{}{{", &self.open_delim)) else {
            return Ok(None);
        };
        let Some(content_len) = remainder.find(&format!("}}{}", &self.close_delim)) else {
            return Err(Error::Parse);
        };
        let content_start = self.pos + self.open_delim.len() + 1;
        Ok(Some((
            Token::Variable(content_start, content_len, false),
            content_len + self.open_delim.len() + self.close_delim.len() + 2,
        )))
    }

    fn scan_tag(&self) -> Result<Option<(Token, usize)>> {
        let Some(remainder) = self.remainder().strip_prefix(&self.open_delim) else {
            return Ok(None);
        };
        let Some(content_len) = remainder.find(&self.close_delim) else {
            return Err(Error::Parse);
        };
        let special_tag_offset = self.pos + self.open_delim.len() + 1;
        let simple_tag_offset = self.pos + self.open_delim.len();
        let token = match remainder.chars().next() {
            Some('#') => Token::SectionStart(special_tag_offset, content_len - 1),
            Some('^') => Token::InvertSectionStart(special_tag_offset, content_len - 1),
            Some('/') => Token::SectionEnd(special_tag_offset, content_len - 1),
            Some('>') => Token::Partial(special_tag_offset, content_len - 1),
            Some('&') => Token::Variable(special_tag_offset, content_len - 1, false),
            Some('!') => Token::Comment,
            _ => Token::Variable(simple_tag_offset, content_len, true),
        };
        let total_delim_len = self.open_delim.len() + self.close_delim.len();
        Ok(Some((token, content_len + total_delim_len)))
    }

    fn scan_text(&self) -> (Token, usize) {
        let len = match self.remainder().find(&self.open_delim) {
            Some(len) => len,
            None => self.remainder().len(),
        };
        (Token::Text(self.pos, len), len)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;

    use super::{Parser, Token::*};

    #[test]
    fn text() -> Result<()> {
        let text = "foo";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(Text(0, 3)));
        Ok(())
    }

    #[test]
    fn unclosed_tag() -> Result<()> {
        let text = "{{foo";
        let mut parser = Parser::new(text);
        let token = parser.next_token();
        assert!(token.is_err());
        Ok(())
    }

    #[test]
    fn variable() -> Result<()> {
        let text = "{{foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(Variable(2, 3, true)));
        Ok(())
    }

    #[test]
    fn unescaped_variable() -> Result<()> {
        let text = "{{&foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(Variable(3, 3, false)));
        Ok(())
    }

    #[test]
    fn unescaped_variable_2() -> Result<()> {
        let text = "{{{foo}}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(Variable(3, 3, false)));
        Ok(())
    }

    #[test]
    fn section_start() -> Result<()> {
        let text = "{{#foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(SectionStart(3, 3)));
        Ok(())
    }

    #[test]
    fn invert_section_start() -> Result<()> {
        let text = "{{^foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(InvertSectionStart(3, 3)));
        Ok(())
    }

    #[test]
    fn section_end() -> Result<()> {
        let text = "{{/foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(SectionEnd(3, 3)));
        Ok(())
    }

    #[test]
    fn comment() -> Result<()> {
        let text = "{{!foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(Comment));
        Ok(())
    }

    #[test]
    fn partial() -> Result<()> {
        let text = "{{>foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(Partial(3, 3)));
        Ok(())
    }

    #[test]
    fn set_delim() -> Result<()> {
        let text = "{{=// //=}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(SetDelim("//".into(), "//".into())));
        Ok(())
    }
}

#[cfg(test)]
mod spec_tests {
    use crate::ast::Node::*;
    use crate::ast::Variable;
    use crate::error::Result;
    use crate::parser::Parser;

    #[test]
    fn typical() -> Result<()> {
        let text = "\
Hello {{name}}
You have just won {{value}} dollars!
{{#in_ca}}
Well, {{taxed_value}} dollars, after taxes.
{{/in_ca}}";
        let ast = Parser::parse(text)?;
        assert_eq!(ast[0], Text("Hello ".into()));
        assert_eq!(ast[1], Variable(Variable::new("name".into(), true)));
        assert_eq!(ast[2], Text("\nYou have just won ".into()));
        assert_eq!(ast[3], Variable(Variable::new("value".into(), true)));
        assert_eq!(ast[4], Text(" dollars!\n".into()));
        assert_eq!(ast[5][0], Text("\nWell, ".into()));
        assert_eq!(
            ast[5][1],
            Variable(Variable::new("taxed_value".into(), true))
        );
        assert_eq!(ast[5][2], Text(" dollars, after taxes.\n".into()));
        Ok(())
    }
}
