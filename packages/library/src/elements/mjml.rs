use super::body::mj_body::{MJBody, NAME as MJ_BODY};
use super::head::mj_head::{MJHead, NAME as MJ_HEAD};
use super::prelude::*;
use super::Error;
use crate::parser::{next_token, MJMLParser};
use crate::util::context::Context;
use crate::Options;
use log::debug;
use xmlparser::{StrSpan, Token, Tokenizer};

#[derive(Clone, Debug)]
pub struct MJMLElement {
    context: Option<Context>,
    head: MJHead,
    body: MJBody,
}

struct MJMLElementParser {
    options: Options,
    context: Option<Context>,
    head: MJHead,
    body: Option<MJBody>,
}

impl MJMLElementParser {
    pub fn new(options: Options) -> Self {
        Self {
            options: options.clone(),
            context: None,
            head: MJHead::empty(options),
            body: None,
        }
    }
}

impl MJMLParser for MJMLElementParser {
    type Output = MJMLElement;

    fn build(mut self) -> Result<Self::Output, Error> {
        let mut body = self.body.unwrap_or_else(MJBody::empty);
        body.set_context(Context::default());
        body.update_header(self.head.get_mut_header());
        Ok(MJMLElement {
            context: self.context,
            head: self.head,
            body,
        })
    }

    fn parse_child_comment(&mut self, _value: StrSpan) -> Result<(), Error> {
        log::warn!("comment ignored in mjml root element");
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer,
    ) -> Result<(), Error> {
        match tag.as_str() {
            MJ_HEAD => {
                self.head = MJHead::parse(tokenizer, self.options.clone())?;
            }
            MJ_BODY => {
                self.body = Some(MJBody::parse(tokenizer, self.head.get_header())?);
            }
            _ => return Err(Error::UnexpectedElement(tag.to_string())),
        };
        Ok(())
    }
}

impl<'a> MJMLElement {
    pub fn parse(tokenizer: &mut Tokenizer<'a>, opts: Options) -> Result<MJMLElement, Error> {
        MJMLElementParser::new(opts).parse(tokenizer)?.build()
    }

    pub fn parse_root(tokenizer: &mut Tokenizer<'a>, opts: Options) -> Result<MJMLElement, Error> {
        let token = next_token(tokenizer)?;
        match token {
            Token::ElementStart {
                prefix: _,
                local,
                span: _,
            } => match local.as_str() {
                "mjml" => Self::parse(tokenizer, opts),
                _ => Err(Error::UnexpectedElement(local.to_string())),
            },
            _ => Err(Error::InvalidChild),
        }
    }

    pub fn get_title(&self) -> String {
        debug!("get_title");
        self.head.get_title()
    }

    pub fn get_preview(&self) -> String {
        debug!("get_preview");
        self.head.get_preview()
    }

    pub fn get_html(&self) -> Result<String, Error> {
        debug!("get_html");
        let header = self.head.get_header();
        Ok(String::from("<!doctype html>")
           + "<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:v=\"urn:schemas-microsoft-com:vml\" xmlns:o=\"urn:schemas-microsoft-com:office:office\">"
           + self.head.render(&header)?.as_str()
           + self.body.render(&header)?.as_str()
           + "</html>")
    }
}
