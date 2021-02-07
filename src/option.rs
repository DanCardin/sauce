use glob::Pattern;

#[derive(Debug)]
pub struct Options<'a> {
    pub globs: Option<Vec<(Option<&'a str>, &'a str)>>,
    pub filters: Option<Vec<(Option<&'a str>, &'a str)>>,
    pub as_: Option<&'a str>,
    pub path: Option<&'a str>,
    pub file: Option<&'a str>,
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Self {
            globs: None,
            filters: None,
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

    pub fn glob_match(&self, kinds: &[&str], value: &str) -> bool {
        check_matches(&self.globs, kinds, value, |g, v| {
            if let Ok(pattern) = Pattern::new(g) {
                if pattern.matches(v) {
                    return true;
                }
            } else {
                eprintln!("Invalid pattern {}", g);
            }
            false
        })
    }

    pub fn filter_match(&self, kinds: &[&str], value: &str) -> bool {
        check_matches(&self.filters, kinds, value, |f, v| f == v)
    }
}

fn parse_match_option(value: Option<&str>) -> Option<Vec<(Option<&str>, &str)>> {
    value.map(|v| {
        v.split(',')
            .map(|raw| {
                let mut iter = raw.splitn(2, ':');
                let first = iter.next();
                let second = iter.next();
                match second {
                    Some(second) => (first, second),
                    None => (None, raw),
                }
            })
            .collect()
    })
}

fn check_matches<F>(
    options: &Option<Vec<(Option<&str>, &str)>>,
    kinds: &[&str],
    value: &str,
    matcher: F,
) -> bool
where
    F: Fn(&str, &str) -> bool,
{
    if let Some(globs) = options {
        for (tag, glob) in globs {
            if let Some(tag) = tag {
                if !kinds.iter().any(|k| k == tag) {
                    continue;
                }
            }

            if matcher(glob, value) {
                return true;
            }
        }
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    mod parse_match_options {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_no_ops_with_none_value() {
            let result = parse_match_option(None);
            assert_eq!(result, None)
        }

        #[test]
        fn it_splits_matches_on_commas() {
            let result = parse_match_option(Some("foo,bar"));
            assert_eq!(result, Some(vec![(None, "foo"), (None, "bar")]))
        }

        #[test]
        fn it_splits_target_and_term() {
            let result = parse_match_option(Some("env:bar"));
            assert_eq!(result, Some(vec![(Some("env"), "bar")]))
        }

        #[test]
        fn it_multiple() {
            let result = parse_match_option(Some("foo,alias:wat"));
            assert_eq!(result, Some(vec![(None, "foo"), (Some("alias"), "wat")]))
        }
    }
}
