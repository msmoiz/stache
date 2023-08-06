use crate::ast::Variant;
use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'t> {
    Text(&'t str),
    Newline(&'t str),
    Whitespace(&'t str),
    Variable(&'t str, bool),
    SectionStart(&'t str, Variant),
    SectionEnd(&'t str),
    Partial(&'t str, String),
    SetDelim(&'t str, &'t str),
    Comment,
}

pub struct Lexer<'t> {
    text: &'t str,
    pos: usize,
    open_delim: &'t str,
    close_delim: &'t str,
}

impl<'t> Lexer<'t> {
    pub fn new(text: &'t str) -> Self {
        Self {
            text,
            pos: 0,
            open_delim: "{{",
            close_delim: "}}",
        }
    }

    pub fn tokens(&mut self) -> Result<Vec<Token<'t>>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next()? {
            tokens.push(token);
        }
        Self::strip_standalone_whitespace(&mut tokens);
        Ok(tokens)
    }

    fn strip_standalone_whitespace<'a>(tokens: &mut Vec<Token<'t>>) {
        let mut ix = 0;
        while let Some(line) = Self::line(&tokens[ix..]) {
            let line_len = line.len();
            let contains_text_or_var = line.iter().any(Self::is_text_or_var);
            let special_tag_count = line.iter().filter(Self::is_special_tag).count();
            if contains_text_or_var || special_tag_count != 1 {
                ix += line_len;
                continue;
            }

            let tag_pos = line.iter().position(|x| Self::is_special_tag(&x)).unwrap();
            if matches!(line[tag_pos], Token::Partial(..)) {
                let mut indent = String::new();
                for token in &line[..tag_pos] {
                    let Token::Whitespace(ws) = token else {
                        unreachable!();
                    };
                    indent.push_str(ws);
                }
                let Token::Partial(_, partial_indent) = &mut tokens[ix + tag_pos] else {
                    unreachable!();
                };
                *partial_indent = indent;
            }

            let newline_pos = ix + line_len - tag_pos;
            tokens.drain(ix..ix + tag_pos);
            tokens.drain(ix + 1..newline_pos);
            ix += 1;
        }
    }

    fn is_text_or_var(token: &Token) -> bool {
        use Token::*;
        matches!(token, Text(_) | Variable(..))
    }

    fn is_special_tag(token: &&Token) -> bool {
        use Token::*;
        matches!(
            token,
            SectionStart(..) | SectionEnd(_) | Partial(..) | SetDelim(..) | Comment
        )
    }

    fn line<'a>(tokens: &'a [Token<'t>]) -> Option<&'a [Token<'t>]> {
        match tokens
            .iter()
            .position(|token| matches!(token, Token::Newline(_)))
        {
            Some(pos) => Some(&tokens[..pos + 1]),
            None => match tokens.len() > 0 {
                true => Some(&tokens),
                false => None,
            },
        }
    }

    fn next(&mut self) -> Result<Option<Token<'t>>> {
        if self.pos == self.text.len() {
            return Ok(None);
        }

        while let Some((token, len)) = self.scan_set_delim()? {
            self.pos += len;
            let Token::SetDelim(open, close) = &token else {
                return Err(Error::Parse);
            };
            self.open_delim = open;
            self.close_delim = close;
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

    fn remainder(&self) -> &'t str {
        &self.text[self.pos..]
    }

    fn scan_set_delim(&self) -> Result<Option<(Token<'t>, usize)>> {
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
            Token::SetDelim(open_delim, close_delim),
            content_len + self.open_delim.len() + self.close_delim.len() + 2,
        )))
    }

    fn scan_triple_unescape(&self) -> Result<Option<(Token<'t>, usize)>> {
        let Some(remainder) = self.remainder().strip_prefix(&format!("{}{{", &self.open_delim)) else {
            return Ok(None);
        };
        let Some(content_len) = remainder.find(&format!("}}{}", &self.close_delim)) else {
            return Err(Error::Parse);
        };
        Ok(Some((
            Token::Variable(remainder[..content_len].trim(), false),
            content_len + self.open_delim.len() + self.close_delim.len() + 2,
        )))
    }

    fn scan_tag(&self) -> Result<Option<(Token<'t>, usize)>> {
        let Some(remainder) = self.remainder().strip_prefix(&self.open_delim) else {
            return Ok(None);
        };
        let Some(content_len) = remainder.find(&self.close_delim) else {
            return Err(Error::Parse);
        };
        let token = match remainder.chars().next() {
            Some('#') => Token::SectionStart(remainder[1..content_len].trim(), Variant::Direct),
            Some('^') => Token::SectionStart(remainder[1..content_len].trim(), Variant::Inverse),
            Some('/') => Token::SectionEnd(remainder[1..content_len].trim()),
            Some('>') => Token::Partial(remainder[1..content_len].trim(), String::new()),
            Some('&') => Token::Variable(remainder[1..content_len].trim(), false),
            Some('!') => Token::Comment,
            _ => Token::Variable(remainder[..content_len].trim(), true),
        };
        let total_delim_len = self.open_delim.len() + self.close_delim.len();
        Ok(Some((token, content_len + total_delim_len)))
    }

    fn scan_newline(&self) -> Option<(Token<'t>, usize)> {
        match self.remainder().strip_prefix("\r\n") {
            Some(_) => Some((Token::Newline("\r\n"), 2)),
            None => match self.remainder().strip_prefix("\n") {
                Some(_) => Some((Token::Newline("\n"), 1)),
                None => None,
            },
        }
    }

    fn scan_whitespace(&self) -> Option<(Token<'t>, usize)> {
        let chars = self.remainder().chars();
        let len = chars.take_while(|x| matches!(x, ' ' | '\t')).count();
        match len > 0 {
            true => Some((Token::Whitespace(&self.remainder()[..len]), len)),
            false => None,
        }
    }

    fn scan_text(&self) -> (Token<'t>, usize) {
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
        assert_eq!(token, Some(Text("foo")));
        Ok(())
    }

    #[test]
    fn newline() -> Result<()> {
        let text = "\n";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Newline("\n")));
        Ok(())
    }

    #[test]
    fn newline_win() -> Result<()> {
        let text = "\r\n";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Newline("\r\n")));
        Ok(())
    }

    #[test]
    fn whitespace() -> Result<()> {
        let text = " \t";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Whitespace(" \t")));
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
        assert_eq!(token, Some(Variable("foo", true)));
        Ok(())
    }

    #[test]
    fn unescaped_variable() -> Result<()> {
        let text = "{{&foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Variable("foo", false)));
        Ok(())
    }

    #[test]
    fn unescaped_variable_2() -> Result<()> {
        let text = "{{{foo}}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(Variable("foo", false)));
        Ok(())
    }

    #[test]
    fn section_start() -> Result<()> {
        let text = "{{#foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(SectionStart("foo", Variant::Direct)));
        Ok(())
    }

    #[test]
    fn invert_section_start() -> Result<()> {
        let text = "{{^foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(SectionStart("foo", Variant::Inverse)));
        Ok(())
    }

    #[test]
    fn section_end() -> Result<()> {
        let text = "{{/foo}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(SectionEnd("foo")));
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
        assert_eq!(token, Some(Partial("foo", String::new())));
        Ok(())
    }

    #[test]
    fn set_delim() -> Result<()> {
        let text = "{{=// //=}}";
        let mut lexer = Lexer::new(text);
        let token = lexer.next()?;
        assert_eq!(token, Some(SetDelim("//", "//")));
        Ok(())
    }
}
