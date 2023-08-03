use crate::ast::Variant;
use crate::error::{Error, Result};

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    Newline(String),
    Whitespace(String),
    Variable(String, bool),
    SectionStart(String, Variant),
    SectionEnd(String),
    Partial(String),
    Comment,
}

pub struct Lexer<'a> {
    text: &'a str,
    pos: usize,
    open_delim: String,
    close_delim: String,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            pos: 0,
            open_delim: String::from("{{"),
            close_delim: String::from("}}"),
        }
    }

    pub fn tokens(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next()? {
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn next(&mut self) -> Result<Option<Token>> {
        if self.pos == self.text.len() {
            return Ok(None);
        }

        while let Some((open, close, len)) = self.scan_set_delim()? {
            self.pos += len;
            self.open_delim = open;
            self.close_delim = close;
        }

        if let Some((token, len)) = self.scan_triple_unescape()? {
            self.pos += len;
            return Ok(Some(token));
        }

        if let Some((token, len)) = self.scan_tag()? {
            self.pos += len;
            return Ok(Some(token));
        }

        if let Some((token, len)) = self.scan_newline() {
            self.pos += len;
            return Ok(Some(token));
        }

        if let Some((token, len)) = self.scan_whitespace() {
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

    fn scan_set_delim(&self) -> Result<Option<(String, String, usize)>> {
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
            open_delim.into(),
            close_delim.into(),
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
        Ok(Some((
            Token::Variable(remainder[..content_len].into(), false),
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
        let token = match remainder.chars().next() {
            Some('#') => Token::SectionStart(remainder[1..content_len].into(), Variant::Direct),
            Some('^') => Token::SectionStart(remainder[1..content_len].into(), Variant::Inverse),
            Some('/') => Token::SectionEnd(remainder[1..content_len].into()),
            Some('>') => Token::Partial(remainder[1..content_len].into()),
            Some('&') => Token::Variable(remainder[1..content_len].into(), false),
            Some('!') => Token::Comment,
            _ => Token::Variable(remainder[..content_len].into(), true),
        };
        let total_delim_len = self.open_delim.len() + self.close_delim.len();
        Ok(Some((token, content_len + total_delim_len)))
    }

    fn scan_newline(&self) -> Option<(Token, usize)> {
        match self.remainder().strip_prefix("\r\n") {
            Some(_) => Some((Token::Newline("\r\n".into()), 2)),
            None => match self.remainder().strip_prefix("\n") {
                Some(_) => Some((Token::Newline("\n".into()), 1)),
                None => None,
            },
        }
    }

    fn scan_whitespace(&self) -> Option<(Token, usize)> {
        let chars = self.remainder().chars();
        let len = chars
            .map_while(|x| matches!(x, ' ' | '\t').then(|| x.len_utf8()))
            .sum();
        if len == 0 {
            return None;
        }
        Some((Token::Whitespace(self.remainder()[..len].into()), len))
    }

    fn scan_text(&self) -> (Token, usize) {
        let mut len = 0;
        while len < self.remainder().len() {
            let text = &self.remainder()[len..];
            if text.starts_with(&self.open_delim)
                || text.starts_with("\r\n")
                || text.starts_with("\n")
            {
                break;
            }
            len += text.chars().next().unwrap().len_utf8();
        }
        (Token::Text(self.remainder()[..len].into()), len)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Variant, error::Result};

    use super::{Lexer, Token::*};

    #[test]
    fn text() -> Result<()> {
        let text = "foo";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Text("foo".into())));
        Ok(())
    }

    #[test]
    fn newline() -> Result<()> {
        let text = "\n";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Newline("\n".into())));
        Ok(())
    }

    #[test]
    fn newline_win() -> Result<()> {
        let text = "\r\n";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Newline("\r\n".into())));
        Ok(())
    }

    #[test]
    fn whitespace() -> Result<()> {
        let text = " \t";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Whitespace(" \t".into())));
        Ok(())
    }

    #[test]
    fn unclosed_tag() -> Result<()> {
        let text = "{{foo";
        let mut lexer = Lexer::new(text);
        let token = lexer.next();
        assert!(token.is_err());
        Ok(())
    }

    #[test]
    fn variable() -> Result<()> {
        let text = "{{foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Variable("foo".into(), true)));
        Ok(())
    }

    #[test]
    fn unescaped_variable() -> Result<()> {
        let text = "{{&foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Variable("foo".into(), false)));
        Ok(())
    }

    #[test]
    fn unescaped_variable_2() -> Result<()> {
        let text = "{{{foo}}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Variable("foo".into(), false)));
        Ok(())
    }

    #[test]
    fn section_start() -> Result<()> {
        let text = "{{#foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(SectionStart("foo".into(), Variant::Direct)));
        Ok(())
    }

    #[test]
    fn invert_section_start() -> Result<()> {
        let text = "{{^foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(SectionStart("foo".into(), Variant::Inverse)));
        Ok(())
    }

    #[test]
    fn section_end() -> Result<()> {
        let text = "{{/foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(SectionEnd("foo".into())));
        Ok(())
    }

    #[test]
    fn comment() -> Result<()> {
        let text = "{{!foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Comment));
        Ok(())
    }

    #[test]
    fn partial() -> Result<()> {
        let text = "{{>foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Partial("foo".into())));
        Ok(())
    }
}
