# YAsemver - Yet Another (somewhat lenient) Semantic Versioning Library

This is my attempt to do a lenient semantic versioning library. It parses both versions and ranges (including basic and somewhat advanced operators (like `~` and `^` )) and can do comparassiion between versions and ranges, checking if a version is inside the other, etc. 

This crate uses the crate [peg](https://crates.io/crates/peg) to generate the parser. The parser is similar to [lenient_semver](https://crates.io/crates/lenient_semver) in things such as letting it only be major or minor and letting it have a starting "v" (`1 == 1.0.0 == 1.0 == 1. == v1`) but it implements ranges, comparasions, and implements some things different from it.

Here's how Version is defined:

```rust
pub struct Version {
  major: u32,
  minor: u32,
  patch: u32,
  extra_version: Option<RadixNum>,
  pre_release: Option<RadixNum>,
  build: Option<String>,
}
```

`extra_version` it's intended to be used for versions like `1.0.0.2`, so `extra_version = 2)`. This makes it so it's different from `pre_release` and has its intended proprieties (such as `1.0.0.2 > 1.0.0`, where if it would be converted to `pre_release` it would be `1.0.0-2 < 1.0.0` [^rangePre]).

[^rangePre]: In this crate, `1.0.0-2 == 1.0.0`, because of intended use cases around ranges, for example, the range `< 2.0` should not include version `2.0-alpha`, even if that's tecnically smaller. To actually compare if one version is older than other, one should use `Version.is_older_than`. This function will not compare builds if all other fields are equal (for that use `Version.is_older_than_with_build`, intended for versions like "1.0.0+97f13" and not "1.0.0+windows (sidenote that stuff should be pre-release not build but most of the times we don't control that so there's the option)).

In this crate, to make comparassions work as intended in `extra_version` and `pre_release`, they are read as base36 number, instead of them just being a string or forcing the field to be numbers only. This makes it so a prerelease can have letters and different caracther sizes, and still compare as intended (if it was a string, "10" would be smaller than "2" or "alpha").

> [!NOTE]
> A downside of this is that if multiple dots are provided in this it can lead to unexpected results, since in the transformation the dots and underscores are removed (eg: 1.0.0.36.2 will be equal to 1.0.0.3.62).[^comp]

Another difference between this and ["Semantic Versioning 2.0.0"](https://semver.org/#semantic-versioning-200) is that build and pre_release can be in any order (`1.0.0+windows-62748 = 1.0.0+62748-windows`). Dots on the build are interpreted as build (`1.0.0+windows.1 -> build = Some("windows.1")`). Comparing the same versions with different builds will lead them to being the same (`1.0.0+windows == 1.0.0+linux`), so if you want to check for full equalities, use `Version.is`.

This crate implements ranges by extrapolating them into a(n inclusive) minimum and a(n exclusive) maximum range, and then checking for exceptions if needed. `"^1.2.3" == ">=1.2.3 <2.0.0"`, `~1.2.3" == ">=1.2.3 <1.0.0"`. All the common operators are included in the crate (except star ranges (eg 1.0.\*)[^comp], "*" is read as Range::any()).

Here's how Range is defined:
```rust
pub struct Range { 
  pub min: Option<Version>, //inclusive
  pub max: Option<Version>, //exclusive, because it's hard to go back to the previous version
  pub except: Vec<Version>, 
  pub include: Vec<Version>
}
```

[^comp]: More complicated to implement/fix and I currenctly don't need it so I'll only implement this if the crate get's enough trafic or with someone's help.

## Serde

This crate implements serde for Version and Range, so you can use it with serde_json, serde_yaml, etc. To enable it, use the feature `serde`.

## Example

```rust
// TODO

```


## TODO before 1.0.0

- [ ] Documentation (the functions are IMO pretty self explanatory, but I should still document them)