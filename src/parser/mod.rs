#![warn(missing_docs)]
//! Tools to execute parser of a expression

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//
//  mod parser
//
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

/// Support for minimum expressions elements
pub mod atom;
pub mod expression;

use std::str::Chars;
use {Error, Possition};

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  T Y P E S
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

//-----------------------------------------------------------------------
#[derive(Debug, Clone)]
pub(crate) struct Status<'a> {
    text2parse: &'a str,
    it_parsing: Chars<'a>,
    pub(crate) pos: Possition,
    pub(crate) rules: &'a expression::SetOfRules<'a>,
}

impl<'a> Status<'a> {
    pub(crate) fn init(t2p: &'a str, rules: &'a expression::SetOfRules) -> Self {
        Status {
            text2parse: t2p,
            it_parsing: t2p.chars(),
            pos: Possition::init(),
            rules: rules,
        }
    }
}

//-----------------------------------------------------------------------
//-----------------------------------------------------------------------
//
//  A P I
//
//-----------------------------------------------------------------------
//-----------------------------------------------------------------------

//-----------------------------------------------------------------------
//  T E S T
//-----------------------------------------------------------------------
#[cfg(test)]
mod test;

//-----------------------------------------------------------------------
//  I N T E R N A L
//-----------------------------------------------------------------------
impl Error {
    pub(crate) fn from_status(status: &Status, descr: &str) -> Self {
        Error {
            pos: status.pos.clone(),
            descr: descr.to_owned(),
            line: "pending".to_owned(),
        }
    }
}
