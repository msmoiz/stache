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

    fn next_token(&mut self) -> Option<Token> {
        if self.pos == self.text.len() {
            return None;
        }

        if let Some((token, len)) = self.scan_section_start() {
            self.pos += len;
            return Some(token);
        }

        if let Some((token, len)) = self.scan_invert_section_start() {
            self.pos += len;
            return Some(token);
        }

        if let Some((token, len)) = self.scan_section_end() {
            self.pos += len;
            return Some(token);
        }

        if let Some((token, len)) = self.scan_comment() {
            self.pos += len;
            return Some(token);
        }

        if let Some((token, len)) = self.scan_partial() {
            self.pos += len;
            return Some(token);
        }

        if let Some((token, len)) = self.scan_set_delim() {
            self.pos += len;
            return Some(token);
        }

        if let Some((token, len)) = self.scan_variable() {
            self.pos += len;
            return Some(token);
        }

        let (token, len) = self.scan_text();
        self.pos += len;
        Some(token)
    }

    fn remainder(&self) -> &str {
        &self.text[self.pos..]
    }

    fn scan_section_start(&self) -> Option<(Token, usize)> {
        let Some(remainder) = self.remainder().strip_prefix("{{#") else {
            return None;
        };
        let Some(len) = remainder.find("}}") else {
            // @todo: should be an error
            return None;
        };
        // @todo: check that variable has proper name
        Some((Token::SectionStart(len), len + 5))
    }

    fn scan_invert_section_start(&self) -> Option<(Token, usize)> {
        let Some(remainder) = self.remainder().strip_prefix("{{^") else {
            return None;
        };
        let Some(len) = remainder.find("}}") else {
            // @todo: should be an error
            return None;
        };
        // @todo: check that variable has proper name
        Some((Token::InvertSectionStart(len), len + 5))
    }

    fn scan_section_end(&self) -> Option<(Token, usize)> {
        let Some(remainder) = self.remainder().strip_prefix("{{/") else {
            return None;
        };
        let Some(len) = remainder.find("}}") else {
            // @todo: should be an error
            return None;
        };
        // @todo: check that variable has proper name
        Some((Token::SectionEnd(len), len + 5))
    }

    fn scan_comment(&self) -> Option<(Token, usize)> {
        let Some(remainder) = self.remainder().strip_prefix("{{!") else {
            return None;
        };
        let Some(len) = remainder.find("}}") else {
            // @todo: should be an error
            return None;
        };
        // @todo: check that variable has proper name
        Some((Token::Comment(len), len + 5))
    }

    fn scan_partial(&self) -> Option<(Token, usize)> {
        let Some(remainder) = self.remainder().strip_prefix("{{>") else {
            return None;
        };
        let Some(len) = remainder.find("}}") else {
            // @todo: should be an error
            return None;
        };
        // @todo: check that variable has proper name
        Some((Token::Partial(len), len + 5))
    }

    fn scan_set_delim(&self) -> Option<(Token, usize)> {
        let Some(remainder) = self.remainder().strip_prefix("{{=") else {
            return None;
        };
        let Some(len) = remainder.find("}}") else {
            // @todo: should be an error
            return None;
        };
        // @todo: check that variable has proper name
        Some((Token::SetDelim(len), len + 5))
    }

    fn scan_variable(&self) -> Option<(Token, usize)> {
        let Some(remainder) = self.remainder().strip_prefix("{{") else {
            return None;
        };
        let Some(close_delim_pos) = remainder.find("}}") else {
            // @todo: should be an error
            return None;
        };
        // @todo: check that variable has proper name
        Some((Token::Variable(close_delim_pos), close_delim_pos + 4))
    }

    fn scan_text(&self) -> (Token, usize) {
        let mut len = 0;
        let mut chars = self.remainder().chars();
        while let Some(ch) = chars.next() {
            // @todo: should stop at double mustache only
            if let '{' = ch {
                break;
            }
            len += ch.len_utf8();
        }
        (Token::Text(self.pos, len), len)
    }
}

#[cfg(test)]
mod tests {
    use super::{Parser, Token::*};

    #[test]
    fn text() {
        let text = "foo";
        let mut parser = Parser::new(text);
        let token = parser.next_token();
        assert_eq!(token, Some(Text(0, 3)));
    }

    #[test]
    fn variable() {
        let text = "{{foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token();
        assert_eq!(token, Some(Variable(3)));
    }

    #[test]
    fn section_start() {
        let text = "{{#foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token();
        assert_eq!(token, Some(SectionStart(3)));
    }

    #[test]
    fn invert_section_start() {
        let text = "{{^foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token();
        assert_eq!(token, Some(InvertSectionStart(3)));
    }

    #[test]
    fn section_end() {
        let text = "{{/foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token();
        assert_eq!(token, Some(SectionEnd(3)));
    }

    #[test]
    fn comment() {
        let text = "{{!foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token();
        assert_eq!(token, Some(Comment(3)));
    }

    #[test]
    fn partial() {
        let text = "{{>foo}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token();
        assert_eq!(token, Some(Partial(3)));
    }

    #[test]
    fn set_delim() {
        let text = "{{=// //}}";
        let mut parser = Parser::new(text);
        let token = parser.next_token();
        assert_eq!(token, Some(SetDelim(5)));
    }
}
