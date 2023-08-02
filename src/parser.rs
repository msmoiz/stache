use crate::error::{Error, Result};

#[derive(Debug, PartialEq)]
enum Token {
    Text(usize, usize),
    Variable(usize),
    SectionStart(usize),
    InvertSectionStart(usize),
    SectionEnd(usize),
    Comment(usize),
    Partial(usize),
    SetDelim(usize),
}

pub struct Parser<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Self { text, pos: 0 }
    }

    pub fn parse(text: &str) {
        let parser = Parser::new(text);
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        if self.pos == self.text.len() {
            return Ok(None);
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

    fn scan_tag(&self) -> Result<Option<(Token, usize)>> {
        let Some(remainder) = self.remainder().strip_prefix("{{") else {
            return Ok(None);
        };
        let Some(len) = remainder.find("}}") else {
            return Err(Error::Parse);
        };
        match remainder.chars().next() {
            Some('#') => Ok(Some((Token::SectionStart(len - 1), len + 5))),
            Some('^') => Ok(Some((Token::InvertSectionStart(len - 1), len + 5))),
            Some('/') => Ok(Some((Token::SectionEnd(len - 1), len + 5))),
            Some('!') => Ok(Some((Token::Comment(len - 1), len + 5))),
            Some('>') => Ok(Some((Token::Partial(len - 1), len + 5))),
            Some('=') => Ok(Some((Token::SetDelim(len - 1), len + 5))),
            _ => Ok(Some((Token::Variable(len), len + 4))),
        }
    }

    fn scan_text(&self) -> (Token, usize) {
        let len = match self.remainder().find("{{") {
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
        assert_eq!(token, Some(Variable(3)));
        Ok(())
    }

    #[test]
    fn section_start() -> Result<()> {
        let text = "{{#foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(SectionStart(3)));
        Ok(())
    }

    #[test]
    fn invert_section_start() -> Result<()> {
        let text = "{{^foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(InvertSectionStart(3)));
        Ok(())
    }

    #[test]
    fn section_end() -> Result<()> {
        let text = "{{/foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(SectionEnd(3)));
        Ok(())
    }

    #[test]
    fn comment() -> Result<()> {
        let text = "{{!foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(Comment(3)));
        Ok(())
    }

    #[test]
    fn partial() -> Result<()> {
        let text = "{{>foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(Partial(3)));
        Ok(())
    }

    #[test]
    fn set_delim() -> Result<()> {
        let text = "{{=// //}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token()?;
        assert_eq!(token, Some(SetDelim(5)));
        Ok(())
    }
}
