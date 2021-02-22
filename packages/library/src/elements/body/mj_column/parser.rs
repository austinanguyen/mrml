use super::MJColumn;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::*;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("direction", "ltr")
        .add("vertical-align", "top");
}

struct MJColumnParser<'h, 'p> {
    header: &'h Header,
    extra: Option<&'p Attributes>,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h, 'p> MJColumnParser<'h, 'p> {
    pub fn new(header: &'h Header, extra: Option<&'p Attributes>) -> Self {
        Self {
            header,
            extra,
            attributes: Attributes::new(),
            children: vec![],
        }
    }
}

impl<'h, 'p> MJMLParser for MJColumnParser<'h, 'p> {
    type Output = MJColumn;

    fn build(self) -> Result<Self::Output, Error> {
        let mut attributes = self.header.default_attributes.concat_attributes(
            super::NAME,
            &DEFAULT_ATTRIBUTES,
            &self.attributes,
        );
        if let Some(extra) = self.extra {
            attributes.merge(extra);
        }
        attributes.merge(&self.attributes);
        Ok(MJColumn {
            attributes,
            context: None,
            children: self.children,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.attributes.set(name, value);
        Ok(())
    }

    fn parse_child_comment(&mut self, value: StrSpan) -> Result<(), Error> {
        self.children.push(BodyElement::comment(value.to_string()));
        Ok(())
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.children.push(BodyElement::text(value.to_string()));
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.children
            .push(BodyElement::parse(tag, tokenizer, self.header, None)?);
        Ok(())
    }
}

impl MJColumn {
    pub fn parse<'a>(
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJColumn, Error> {
        MJColumnParser::new(header, extra).parse(tokenizer)?.build()
    }
}
