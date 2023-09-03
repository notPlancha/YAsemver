use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use thiserror::Error;
use crate::parsing::grammer::the_parser::{parse_version, parse_range};


#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
  #[error("error in parsing version")]
  InvalidVersion,
  #[error("error in parsing range")]
  InvalidRange,
}


#[derive(Debug, Clone, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub struct Version {
  major: u32,
  minor: u32,
  patch: u32,
  extra_version: Option<String>,
  pre_release: Option<String>,
  build: Option<String>,
}

impl Version {
  pub fn parse<S:Into<String>>(version: S) -> Result<Self, ParseError> {
    let version: Self = parse_version(&version.into()).map_err(|_| ParseError::InvalidVersion)?;
    Ok(version)
  }
  // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d78be90c82a7b80c949f30b5befcd6c2
  pub fn new_w_extra<S:Into<String>>(
    major: u32,
    minor: u32,
    patch: u32,
    //1.1.0.1.5 < 1.1.0.1.6, 1.1.0.1.5 > 1.1.0, 1.1.0.0.0 > 1.1.0
    extra_version: Option<S>,
    // 1.1.0-rc.1 < 1.1.0-rc.2, 1-a < 1-b, 1.1.0-rc.1 <= 1.1.0
    // # Pre-release-note
    // é menor que ele mas no range é igual, tipo uma espécie de epsilon
    // isto é porque o range espera-se que por exemplo >= 1.0, < 2.0 não inclua 2.0-alpha
    // embora tecnicamente inclui pq é antes
    // ainda assim quando for para comparar versões, 2.0-alpha é menor que 2.0 na mesma (por exemplo pra atualizar)
    pre_release: Option<S>,
    //1.1.0+build.1 = 1.1.0+build.2, 1.1.0+build.1 = 1.1.0
    build: Option<S>
  ) -> Self {
    String::from("Hello").to_string();
    Self {
      major,
      minor,
      patch,
      extra_version: extra_version.map(|s| s.into()),
      pre_release: pre_release.map(|s| s.into()),
      build: build.map(|s| s.into()),
    }
  }


  /// Use new_w_extra if you need build or pre_release or extra_version
  /// Or use new(...).change_...()
  pub fn new(major: u32, minor: u32, patch: u32) -> Self {
    Self {
      major,
      minor,
      patch,
      extra_version: None,
      pre_release: None,
      build: None,
    }
  }

  pub const fn new_const(major: u32, minor: u32, patch: u32) -> Self {
    Self {
      major,
      minor,
      patch,
      extra_version: None,
      pre_release: None,
      build: None,
    }
  }

  //could be a cool macro
  //maybe remove if not used anywhere
  /// These are useful to clone a version and change only one of the fields
  pub fn with_major(&self, major: u32) -> Self {
    Self {
      major,
      minor: self.minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  pub fn with_minor(&self, minor: u32) -> Self {
    Self {
      major: self.major,
      minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  pub fn with_patch(&self, patch: u32) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  pub fn with_extra_version(&self, extra_version: Option<impl Into<String>>) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch: self.patch,
      extra_version: extra_version.map(|s| s.into()),
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  pub fn with_extra(&self, extra_version: Option<impl Into<String>>) -> Self {
    self.with_extra_version(extra_version)
  }
  pub fn with_pre_release(&self, pre_release: Option<impl Into<String>>) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release: pre_release.map(|s| s.into()),
      build: self.build.clone(),
    }
  }
  pub fn with_pre(&self, pre_release: Option<impl Into<String>>) -> Self {
    self.with_pre_release(pre_release)
  }
  pub fn with_build(&self, build: Option<impl Into<String>>) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build: build.map(|s| s.into()),
    }
  }


  /// These are useful to change only one of the fields, without cloning
  pub fn major(&mut self, major: u32) -> &mut Self {
    self.major = major;
    self
  }
  pub fn minor(&mut self, minor: u32) -> &mut Self {
    self.minor = minor;
    self
  }
  pub fn patch(&mut self, patch: u32) -> &mut Self {
    self.patch = patch;
    self
  }
  pub fn extra_version(&mut self, extra_version: Option<impl Into<String>>) -> &mut Self {
    self.extra_version = extra_version.map(|s| s.into());
    self
  }
  pub fn extra(&mut self, extra_version: Option<impl Into<String>>) -> &mut Self {
    self.extra_version(extra_version)
  }
  pub fn pre_release(&mut self, pre_release: Option<impl Into<String>>) -> &mut Self {
    self.pre_release = pre_release.map(|s| s.into());
    self
  }
  pub fn pre(&mut self, pre_release: Option<impl Into<String>>) -> &mut Self {
    self.pre_release(pre_release)
  }
  pub fn build(&mut self, build: Option<impl Into<String>>) -> &mut Self {
    self.build = build.map(|s| s.into());
    self
  }




  fn is(&self, other: &Self) -> bool {
    // comparasion with everything, and not equivelant
    self.major == other.major
      && self.minor == other.minor
      && self.patch == other.patch
      && self.extra_version == other.extra_version
      && self.pre_release == other.pre_release
      && self.build == other.build
  }
  fn is_older_than(&self, other: &Self) -> bool {
    // comparasion with everything, different than <= since pre_release is checked
    // check version_parser.rs#Pre-release-note
    self.major < other.major
      || self.minor < other.minor
      || self.patch < other.patch
      || self.extra_version < other.extra_version
      || self.pre_release < other.pre_release
  }
}
impl FromStr for Version {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parse(s)
  }
}
impl FromStr for Range {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parse(s)
  }
}

impl PartialOrd<Version> for Version {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.major < other.major {
      Some(Ordering::Less)
    } else if self.major > other.major {
      Some(Ordering::Greater)
    } else if self.minor < other.minor {
      Some(Ordering::Less)
    } else if self.minor > other.minor {
      Some(Ordering::Greater)
    } else if self.patch < other.patch {
      Some(Ordering::Less)
    } else if self.patch > other.patch {
      Some(Ordering::Greater)
    } else if self.extra_version < other.extra_version { //TODO check if this works since it's optional
      Some(Ordering::Less)
    } else if self.extra_version > other.extra_version {
      Some(Ordering::Greater)
      // pre-release isn't checked because this is for implmenting ranges, see version_parser.rs#Pre-release-note
    } else {
      Some(Ordering::Equal)
    }
  }
}
impl Ord for Version {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut s = format!("{}.{}.{}", self.major, self.minor, self.patch);
    if let Some(extra_version) = &self.extra_version {
      s.push_str(&format!(".{}", extra_version));
    }
    if let Some(pre_release) = &self.pre_release {
      s.push_str(&format!("-{}", pre_release));
    }
    if let Some(build) = &self.build {
      s.push_str(&format!("+{}", build));
    }
    write!(f, "{}", s)
  }
}

// impl Display for Version {
//   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//     write!(f, "{}", self.to_string())
//   }
// }
impl Default for Version {
  fn default() -> Self {
    Self {
      major: 1,
      minor: 0,
      patch: 0,
      extra_version: None,
      pre_release: None,
      build: None,
    }
  }
}

#[derive(Debug, Default, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)] //default should be equal to *
// debug is needed for assert_eq
pub struct Range {
pub min: Option<Version>, //inclusive
pub max: Option<Version>, //exclusive, because it's hard to go back to the previous version
pub except: Vec<Version>,
  pub include: Vec<Version>
}

impl Display for Range {

  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.is_any() {
      return write!(f, "*");
    }
    // special check to prefer ^ and ~ when possible
    if let Some(caret) = self.to_caret() {
      return write!(f, "{}", caret);
    }
    if let Some(tilde) = self.to_tilde() {
      return write!(f, "{}", tilde);
    }

    let mut s = String::new();
    if let Some(min) = &self.min {
      s.push_str(&format!(">={},", min.to_string()));
    }
    if let Some(max) = &self.max {
      s.push_str(&format!("<{},", max.to_string()));
    }
    for except in &self.except {
      s.push_str(&format!("!={},", except.to_string()));
    }
    for include in &self.include {
      s.push_str(&format!("={},", include.to_string()));
    }
    s.pop(); //remove the last comma
    write!(f, "{}", s)
  }
}


impl Range {
  fn contains(&self, version: Version) -> bool {
    todo!()
  }
  fn is_any(&self) -> bool { // is empty or is just >= 0.0.0
    (self.min.clone().is_none() || self.min.clone().unwrap() == Version::new(0, 0, 0))
      && self.max.is_none()
      && self.except.is_empty()
      && self.include.is_empty()
    //maybe when include is something and min is 0.0.0 it should any but it's fine
  }
  fn is_valid(&self) -> bool { // is not empty and min <= max and is not < 0.0.0
    todo!()
  }
  fn is_exact_match(&self) -> bool { // min == max or just includes one version
    todo!()
  }
  fn to_caret(&self) -> Option<String> {
    //transforms range to caret range if appropriate
    if self.min.is_some() && self.max.is_some() {
      let min = self.min.as_ref().unwrap();
      let max= self.max.as_ref().unwrap();
      if max.patch == 0 && max.minor == 0 && max.major == min.major + 1 {
        return Some(format!("^{}", min));
      }
    }
    return None
  }

  fn to_tilde(&self) -> Option<String> {
    //transforms range to tilde range if appropriate
    if self.min.is_some() && self.max.is_some() {
      let min = self.min.clone().unwrap();
      let max = self.max.clone().unwrap();
      if max.patch == 0 && max.minor == min.minor + 1 && max.major == min.major {
        return Some(format!("~{}", min));
      }
    }
    return None
  }
  fn separate_ops(ranges: Vec<(Op, Version)>) -> HashMap<Op, Vec<Version>> {
    let mut map = HashMap::new();
    for (op, version) in ranges {
      map.entry(op).or_insert_with(Vec::new).push(version);
    }
    map
  }

  pub fn from_ver_vec(ranges: Vec<(Op, Version)>) -> Self {
    // Sort the ranges by version number
    let mut ranges:Vec<(Op, Version)> = Self::sort_vec(ranges);
    // separate the ranges by operator
    let mut map:HashMap<Op, Vec<Version>> = Self::separate_ops(ranges);
    // atribute the ranges to the correct fields
    let min:Option<Version> = (*map.get(&Op::Ge).unwrap_or(&vec![])).first().cloned();
    let max:Option<Version> = (*map.get(&Op::Lt).unwrap_or(&vec![])).last().cloned();
    let except = map.get(&Op::Ne).unwrap_or(&vec![]).clone();
    let include = map.get(&Op::Eq).unwrap_or(&vec![]).clone();
    Range { //Note: this can return an invalid range, that's why we have is_valid
      min,
      max,
      except,
      include,
    }
  }
  fn mixed_vec_to_stand_vec(ranges: Vec<(Op, Version)>) -> Vec<(Op, Version)> {
    // Expand tilde, caret, le and gt ranges to simple lt and ge ranges
    ranges.into_iter().flat_map(|(op, version)| {
      match op {
        Op::Tilde => Self::tilde_range_to_vec(version),
        Op::Caret => Self::caret_range_to_vec(version),
        Op::Le => Self::le_range_to_vec(version),
        Op::Gt => Self::gt_range_to_vec(version),
        _ => vec![(op, version)],
      }
    }).collect::<Vec<_>>()
  }

  fn sort_vec(ranges: Vec<(Op, Version)>) -> Vec<(Op, Version)> {
    // Expand tilde, caret, le and gt ranges to simple lt and ge ranges, and sort them ranges by version number,

    let mut ranges = Self::mixed_vec_to_stand_vec(ranges);
    ranges.sort_by(|(_, a), (_, b)| a.cmp(&b));
    ranges
  }

  pub fn parse(range: &str) -> Result<Self, ParseError> {
    let range : Self = parse_range(range).map_err(|_| ParseError::InvalidRange)?;
    Ok(range)
  }

  fn tilde_range_to_vec(version: Version) -> Vec<(Op, Version)> {
    // ~1.2.3 -> >=1.2.3 <1.3.0
    // ~1.2 -> >=1.2.0 <1.3.0
    // ~1 -> >=1.0.0 <1.1.0, since 1 = 1.0.0
    vec![
      (Op::Ge, version.clone()),
      (Op::Lt, Version::new(version.major, version.minor + 1, 0)),
    ]
  }
  fn caret_range_to_vec(version: Version) -> Vec<(Op, Version)> {
    // ^1.2.3 -> >=1.2.3 <2.0.0
    // ^1.2 -> >=1.2.0 <2.0.0
    // ^1 -> >=1.0.0 <2.0.0, since 1 = 1.0.0
    vec![
      (Op::Ge, version.clone()),
      (Op::Lt, Version::new(version.major + 1, 0, 0)),
    ]
  }
  fn le_range_to_lt(version: Version) -> Vec<(Op, Version)> {
    // <=1.2.3 -> <1.2.4
    // <=1.2 -> <1.2.1
    // <=1 -> <1.0.1
    vec![
      (Op::Lt, Version::new(version.major, version.minor, version.patch + 1)),
    ]
  }

  fn le_range_to_vec(version:Version) ->  Vec<(Op, Version)> {Self::le_range_to_lt(version)}

  fn gt_range_to_ge(version: Version) -> Vec<(Op, Version)> {
    // >1.2.3 -> >=1.2.4
    // >1.2 -> >=1.2.1
    // >1 -> >=1.0.1
    vec![
      (Op::Ge, Version::new(version.major, version.minor, version.patch + 1)),
    ]
  }

  fn gt_range_to_vec(version:Version) ->  Vec<(Op, Version)> {Self::gt_range_to_ge(version)}
}

#[derive(PartialEq, Eq, Hash)]
pub enum Op {
  Eq,    // ==
  Ne,    // !=
  Gt,    // >
  Lt,    // <
  Ge,    // >=
  Le,    // <=
  Tilde, // ~
  Caret  // ^
}

impl Op {
  pub fn from_str(op: &str) -> Result<Self, ParseError> {
    match op {
      "==" | "=" | "" => Ok(Self::Eq),
      "!=" => Ok(Self::Ne),
      ">" => Ok(Self::Gt),
      "<" => Ok(Self::Lt),
      ">=" => Ok(Self::Ge),
      "<=" => Ok(Self::Le),
      "~" => Ok(Self::Tilde),
      "^" => Ok(Self::Caret),
      _ => Err(ParseError::InvalidRange)
    }
  }
}