pub struct Source<'a> {
  pub raw: &'a str,
  pub path: &'a str,
}

impl<'a> Source<'a> {
  pub fn new(path: &'a str, raw: &'a str) -> Source<'a> {
    Source { path, raw }
  }
}
