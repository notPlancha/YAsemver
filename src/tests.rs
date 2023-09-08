use radix::RadixErr;
use crate::implementations::{Range, Version};
#[test]
fn parse_ver() -> Result<(), RadixErr> {
  let m = Version::new(1, 2, 3);
  assert_eq!(v("1.2.3"), m);
  assert_eq!(v("1.2.3-alpha"), m.with_pre_release(Some("alpha"))?);
  assert_eq!(v("1.2.3+build"), m.with_build(Some("build")));
  assert_eq!(v("1.2.3-alpha+build"), m.with_pre_release(Some("alpha"))?.with_build(Some("build")));
  assert_eq!(v("1.2.3-alpha.1+build.1"), m.with_pre_release(Some("alpha.1"))?.with_build(Some("build.1")));
  assert_eq!(v("1.2.3.45"), m.with_extra(Some("45"))?);
  assert_eq!(v("1.2.3.43-alpha.1+build.1"), Version::new_w_extra(1, 2, 3, Some("43"), Some("alpha.1"), Some("build.1"))?);
  assert_eq!(v("1.2.3.43+windows-alpha.1"), m.with_extra(Some("43"))?.build(Some("windows")).pre(Some("alpha.1"))?.to_owned());

  assert!(Version::parse("").is_err());
  assert!(Version::parse(" ").is_err());
  assert!(Version::parse("Version 1").is_err());
  assert!(Version::parse("Version-1.2.1").is_err());
  assert_eq!(v("1"), Version::new(1, 0, 0));
  assert_eq!(v("V1"), Version::new(1, 0, 0));
  assert_eq!(v("1.2"), Version::new(1, 2, 0));
  assert_eq!(v("1+build.1"), Version::new(1, 0, 0).build(Some("build.1")).to_owned());
  assert!(Version::parse("1+windows.1+debian").is_err());
  assert!(Version::parse("-1.2.3").is_err());
  assert!(Version::parse("+1.2.3").is_err());
  Ok(())
}
fn v(version: &str) -> Version {
  dbg!(version);
  Version::parse(version).unwrap_or_else(|_| panic!("Failed to parse version: {}", version))
}
#[test]
fn parse_rang() {
  const V: Version = Version::new_const(1, 2, 3);
  assert_eq!(r(">=1.2.3"), Range {
    min: Some(V),
    ..Default::default()
  });
  assert_eq!(r(">1.2.3"), Range {
    min: Some(V.with_patch(4)),
    ..Default::default()
  });
  assert_eq!(r("<1.2.3"), Range {
    max: Some(V),
    ..Default::default()
  });
  assert_eq!(r("<=1.2.3"), Range {
    max: Some(V.with_patch(4)),
    ..Default::default()
  });
  assert_eq!(r("1.2.3"), Range {
    include: vec![V],
    ..Default::default()
  });
  assert_eq!(r("=1.2.3"), Range {
    include: vec![V],
    ..Default::default()
  });
  assert_eq!(r("==1.2.3"), Range {
    include: vec![V],
    ..Default::default()
  });
  assert_eq!(r("== 1.2.3"), Range {
    include: vec![V],
    ..Default::default()
  });
  assert_eq!(r("~1.2.3"), Range {
    min: Some(V),
    max: Some(V.with_minor(3).patch(0).to_owned()),
    ..Default::default()
  });
  assert_eq!(r("~1"), Range {
    min: Some(Version::new(1, 0, 0)),
    max: Some(Version::new(1, 1, 0)),
    ..Default::default()
  });
  assert_eq!(r("~1.2"), Range {
    min: Some(Version::new(1, 2, 0)),
    max: Some(Version::new(1, 3, 0)),
    ..Default::default()
  });
  // assert_eq!(r("~1.2.3-alpha"), Range { // I actually am not sure abt what to do with this TODO
  //   min: Some(V.with_pre_release(Some("alpha"))),
  //   max: Some(V.with_minor(3).patch(0).to_owned()),
  //   ..Default::default()
  // });
  assert_eq!(r("^1.2.3"), Range {
    min: Some(V),
    max: Some(Version::new(2, 0, 0)),
    ..Default::default()
  });
  assert_eq!(r("^ 1.2.3"), Range {
    min: Some(V),
    max: Some(Version::new(2, 0, 0)),
    ..Default::default()
  });

  // multiple versions
  assert_eq!(r(">1.2.3 <1.2.5"), Range {
    min: Some(Version::new(1, 2, 4).to_owned()),
    max: Some(Version::new(1, 2, 5).to_owned()),
    ..Default::default()
  });
  assert_eq!(r("1.2.3 1.2.4"), Range {
    include: vec![Version::new(1, 2, 3).to_owned(), Version::new(1, 2, 4).to_owned()],
    ..Default::default()
  });
  assert_eq!(r(">=1.2.3 <=1.2.5"), Range {
    min: Some(Version::new(1, 2, 3).to_owned()),
    max: Some(Version::new(1, 2, 6).to_owned()),
    ..Default::default()
  });
  assert_eq!(r(">=1.2.3,<=1.2.5"), Range {
    min: Some(Version::new(1, 2, 3).to_owned()),
    max: Some(Version::new(1, 2, 6).to_owned()),
    ..Default::default()
  });
  assert_eq!(r(">=1.2.3, <=1.2.5"), Range {
    min: Some(Version::new(1, 2, 3).to_owned()),
    max: Some(Version::new(1, 2, 6).to_owned()),
    ..Default::default()
  });
  assert_eq!(r(">=1.2.3, <=1.2.5, 1.2.7"), Range {
    min: Some(Version::new(1, 2, 3).to_owned()),
    max: Some(Version::new(1, 2, 6).to_owned()),
    include: vec![Version::new(1, 2, 7).to_owned()],
    ..Default::default()
  });
  assert_eq!(r(">=1.2.3, <=1.2.5, !=1.2.7"), Range {
    min: Some(Version::new(1, 2, 3).to_owned()),
    max: Some(Version::new(1, 2, 6).to_owned()),
    except: vec![Version::new(1, 2, 7).to_owned()],
    ..Default::default()
  });
  assert_eq!(r("*"), Range::any());
  assert!(r("*").is_any())
  //TODO make more extensive tests
}
fn r(range: &str) -> Range {
  dbg!(range);
  Range::parse(range).unwrap_or_else(|_| panic!("Failed to parse range: {}", range))
}

#[test]
fn special_parse_range() {
  // here it's some tests about claims made in the readme and other places
  // maybe there's some repeated but that's alright
  let one = Version::new(1, 0, 0);
  assert_eq!(v("1"), one);
  assert_eq!(v("1.0.0"), one);
  assert_eq!(v("1.0"), one);
  assert_eq!(v("1."), one);
  assert_eq!(v("v1"), one);
  assert_eq!(v("1.0.0-alpha"), one);
  assert!(v("1.0.0-alpha").is_older_than(&one));
  assert_eq!(v("1.0.0+windows"), v("1.0.0+linux"));
  assert!(!v("1.0.0+windows").is(&v("1.0.0+linux")));
  assert!(v("1.0.0.2") > v("1.0.0.0"));
  assert!(v("1.0.0.2") > v("1.0.0"));
  assert_eq!(v("1.0.0+windows-62748"), v("1.0.0+62748-windows"));
  assert_eq!(v("1.0.0+windows.1").build, Some("windows.1".to_string()));
  assert!(v("1.0.0-62747").is_older_than(&v("1.0.0-62748")));
  assert!(v("1.0.0-62747+12345").is_older_than_with_build(&v("1.0.0-62748+12345")));
}