#![allow(dead_code)]

use super::algorithm::Algorithm;
use super::algorithm::BeginToken;
use super::algorithm::BreakToken;
use super::algorithm::Breaks;
use super::algorithm::SIZE_INFINITY;
use std::borrow::Cow;
use std::result;

pub type Result = result::Result<(), Error>;

pub struct Error;

/// Deparse node context: parent / field / identifier / constant.
#[derive(Default)]
pub enum Context {
    #[default]
    None,
    InsertRelation,
    InsertOnConflict,
    Update,
    Returning,
    AExpr,
    Xmlattributes,
    Xmlnamespaces,
    CreateType,
    AlterType,
    SetStatement,
    ForeignRelation,
    Identifier,
    Constant,
}

pub trait Print {
    fn print(&self, p: &mut super::Printer) -> Result {
        self.print_in_context(p, &Context::default())
    }

    fn print_in_context(&self, _p: &mut super::Printer, _ctx: &Context) -> Result {
        Ok(())
    }
}

pub struct Printer {
    inner: Algorithm,
}

impl Printer {
    pub fn new() -> Self {
        Self {
            inner: Algorithm::new(),
        }
    }

    pub fn eof(self) -> String {
        self.inner.eof()
    }

    pub fn offset(&mut self, offset: isize) {
        self.inner.offset(offset)
    }

    pub fn ibox(&mut self, indent: isize) {
        self.inner.scan_begin(BeginToken {
            offset: indent,
            breaks: Breaks::Inconsistent,
        });
    }

    pub fn cbox(&mut self, indent: isize) {
        self.inner.scan_begin(BeginToken {
            offset: indent,
            breaks: Breaks::Consistent,
        });
    }

    pub fn end(&mut self) {
        self.inner.scan_end();
    }

    pub fn word<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        let s = wrd.into();
        self.inner.scan_string(s);
    }

    pub fn keyword<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        self.word(wrd.into().to_ascii_uppercase());
    }

    pub fn keyword_if<S: Into<Cow<'static, str>>>(&mut self, wrd: S, cnd: bool) {
        if cnd {
            self.keyword(wrd);
        }
    }

    pub fn ident<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        self.word(wrd);
    }

    pub fn word_if<S: Into<Cow<'static, str>>>(&mut self, wrd: S, cnd: bool) {
        if cnd {
            self.word(wrd);
        }
    }

    fn spaces(&mut self, n: usize) {
        self.inner.scan_break(BreakToken {
            blank_space: n,
            ..BreakToken::default()
        });
    }

    pub fn zerobreak(&mut self) {
        self.spaces(0);
    }

    pub fn space(&mut self) {
        self.spaces(1);
    }

    pub fn nbsp(&mut self) {
        self.word(" ");
    }

    pub fn hardbreak(&mut self) {
        self.spaces(SIZE_INFINITY as usize);
    }

    pub fn space_if_nonempty(&mut self) {
        self.inner.scan_break(BreakToken {
            blank_space: 1,
            if_nonempty: true,
            ..BreakToken::default()
        });
    }

    pub fn hardbreak_if_nonempty(&mut self) {
        self.inner.scan_break(BreakToken {
            blank_space: SIZE_INFINITY as usize,
            if_nonempty: true,
            ..BreakToken::default()
        });
    }

    pub fn trailing_comma(&mut self, is_last: bool) {
        if is_last {
            self.inner.scan_break(BreakToken {
                pre_break: Some(','),
                ..BreakToken::default()
            });
        } else {
            self.word(",");
            self.space();
        }
    }

    pub fn trailing_comma_or_space(&mut self, is_last: bool) {
        if is_last {
            self.inner.scan_break(BreakToken {
                blank_space: 1,
                pre_break: Some(','),
                ..BreakToken::default()
            });
        } else {
            self.word(",");
            self.space();
        }
    }

    pub fn comma(&mut self, is_last: bool) {
        if !is_last {
            self.word(",");
            self.space();
        }
    }

    pub fn neverbreak(&mut self) {
        self.inner.scan_break(BreakToken {
            never_break: true,
            ..BreakToken::default()
        });
    }
}
