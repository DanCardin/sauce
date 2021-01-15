pub struct GlobalOptions<'a> {
    pub as_: Option<&'a str>,
    pub globs: &'a [&'a str],
    pub filters: &'a [&'a str],
}

impl<'a> GlobalOptions<'a> {
    pub fn new(as_: Option<&'a str>, globs: &'a [&str], filters: &'a [&str]) -> Self {
        Self {
            as_,
            globs,
            filters,
        }
    }
}
