# YAsemver - Yet Another (somewhat lenient) Semantic Versioning Library

This is my attempt to do a lenient semantic versioning library. It parses both versions and ranges (including basic and somewhat advanced operators (like `~` and `^` )) and can do comparassiion between versions and ranges, checking if a version is inside the other, etc. 

Here's how Version is defined:

```rust
pub struct Version {
  major: u32,
  minor: u32,
  patch: u32,
  extra_version: Option<String>,
  pre_release: Option<String>,
  build: Option<String>,
}
```
`extra_version` it's intended to be used for versions like `1.0.0.2`, where it would be `Some("2".to_string())`. This makes it so it's different from `pre_release` and has its intended proprieties (such as `1.0.0.2 > 1.0.0`, where if it would be converted to pre_release it would be `1.0.0-2 < 1.0.0`). Another difference between ["Semantic Versioning 2.0.0"](https://semver.org/#semantic-versioning-200) is that build and pre_release can be in any order (`1.0.0+windows-62748 = 1.0.0+62748-windows`). Dots on the build are interpreted as build (`1.0.0+windows.1` leads to build as `Some("windows.1")`) On the same note, build is comparable too (`1.0.0-62748 > 1.0.0-62747`). 

It implements ranges by extrapolating them into a(n inclusive) minimum and a(n exclusive) maximum range, and then checking for exceptions if needed. `"^1.2.3" == ">=1.2.3 <2.0.0"`, `~1.2.3" == ">=1.2.3 <1.0.0"`

```rust
pub struct Range { 
  pub min: Option<Version>, //inclusive
  pub max: Option<Version>, //exclusive, because it's hard to go back to the previous version
  pub except: Vec<Version>, 
  pub include: Vec<Version>
}
```

This package uses the crate [peg](https://crates.io/crates/peg) to generate the parser.

## Example

```rust
// TODO

```


## TODO before 1.0.0

* Documentation