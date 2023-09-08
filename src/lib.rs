mod implementations;
#[cfg(test)]
mod tests;

use thiserror::Error;
use crate::implementations::{Version, Range, Op};
pub type ParseError = peg::error::ParseError<peg::str::LineCol>;


#[derive(Debug, PartialEq)]
pub struct Dependency {
  pub name: String,
  pub range: Range
}

peg::parser!( pub grammar Parser() for str {
  pub rule parse_version() -> Version
    = " "* v:version() " "* ![_] {v} // ![_] means end of file

  rule version() -> Version
    = ['v' | 'V']? " "? m:main() e:extra()? a:afterV() {
      Version::new_w_extra(
        m.0,
        m.1.unwrap_or(0),
        m.2.unwrap_or(0),
        e,
        a.0,
        a.1
      ).unwrap()
  }
  // pre and build any order and existence
  rule afterV() -> (Option<String>, Option<String>)
    // here end of file is kinda needed because if not it will accept afterV if the order is b p, cause "+window-alpha" will return (None, Some("window")) and come back without checking further
    = p:pre()? b:build()? supOrEnd() { (p, b) }
    / b:build() p:pre() supOrEnd() { (Some(p), Some(b)) }
  rule num() -> u32
    = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }


  rule chars() -> String
    = n:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.']+) {? Ok(n.to_string().to_lowercase())}


  rule separator() -> ()
    = n:$([' ' | ',' | ';']) {}
  rule supOrEnd() -> ()
    = separator()+ {} //* means 0 or more, + means 1 or more
    / ![_] {}

  rule main() -> (u32, Option<u32>, Option<u32>)
    = M:num() "."? m:num()? "."? p:num()? { (M, m, p) }

  rule extra() -> String
    = "." c:chars() { c }

  rule build() -> String
    = "+" c:chars() { c }

  rule pre() -> String
    = "-" c:chars() { c }

  // TODO implement star version (1.0.*) if enough traffic
  // TODO add support for *+build for things like *+windows
  pub rule parse_range() -> Range

    = " "* "*" " "* ![_] { Range::any() }
    / " "* r:(range() ** "") " "* ![_] { Range::from_ver_vec(r) }

  rule range() -> (Op, Version)
    = o:op() " "* v:version() " "* { (o,v) }

  rule op() -> Op
    = o:$("==" / "!=" / "<=" / ">=" / "=" / "<" / ">" / "~" / "^" / " " / "") { Op::from_str(o).unwrap() }
      // => and =< will fail, but that's ok
});