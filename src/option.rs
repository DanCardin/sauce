use crate::filter::parse_match_option;

#[derive(Debug)]
pub struct Options<'a> {
    pub globs: Vec<(Option<&'a str>, &'a str)>,
    pub filters: Vec<(Option<&'a str>, &'a str)>,
    pub as_: Option<&'a str>,
    pub path: Option<&'a str>,
    pub file: Option<&'a str>,
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Self {
            globs: Vec::new(),
            filters: Vec::new(),
            as_: None,
            path: None,
            file: None,
        }
    }
}

impl<'a> Options<'a> {
    pub fn new(
        glob: Option<&'a str>,
        filter: Option<&'a str>,
        as_: Option<&'a str>,
        path: Option<&'a str>,
        file: Option<&'a str>,
    ) -> Self {
        let globs = parse_match_option(glob);
        let filters = parse_match_option(filter);

        Self {
            as_,
            path,
            globs,
            filters,
            file,
        }
    }
}
