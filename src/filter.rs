use glob::Pattern;

pub type MatchOption<'a> = (Option<&'a str>, &'a str);

pub struct FilterOptions<'a> {
    pub globs: &'a [MatchOption<'a>],

    pub filters: &'a [MatchOption<'a>],
    pub filter_exclusions: &'a [MatchOption<'a>],
}

impl<'a> Default for FilterOptions<'a> {
    fn default() -> Self {
        Self {
            globs: &[],
            filters: &[],
            filter_exclusions: &[],
        }
    }
}

pub fn glob_match(globs: &[MatchOption], kinds: &[&str], value: &str) -> bool {
    check_matches(
        globs,
        kinds,
        value,
        |g, v| {
            if let Ok(pattern) = Pattern::new(g) {
                if pattern.matches(v) {
                    return true;
                }
            } else {
                eprintln!("Invalid pattern {}", g);
            }
            false
        },
        true,
    )
}

pub fn filter_match(filters: &[MatchOption], kinds: &[&str], value: &str) -> bool {
    check_matches(filters, kinds, value, |f, v| f == v, true)
}

pub fn filter_exclude(filters: &[MatchOption], kinds: &[&str], value: &str) -> bool {
    check_matches(filters, kinds, value, |f, v| f == v, false)
}

fn check_matches<'a, F>(
    globs: &[MatchOption<'a>],
    kinds: &[&str],
    value: &str,
    matcher: F,
    match_returns: bool,
) -> bool
where
    F: Fn(&str, &str) -> bool,
{
    if globs.is_empty() {
        return true;
    }

    globs.iter().any(|(tag, glob)| {
        if let Some(tag) = tag {
            if !kinds.iter().any(|k| k == tag) {
                return false;
            }
        }

        matcher(glob, value)
    }) == match_returns
}

pub fn parse_match_option(value: Option<&str>) -> Vec<MatchOption> {
    if let Some(value) = value {
        value
            .split(',')
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
    } else {
        Vec::new()
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
            assert_eq!(result, vec![])
        }

        #[test]
        fn it_splits_matches_on_commas() {
            let result = parse_match_option(Some("foo,bar"));
            assert_eq!(result, vec![(None, "foo"), (None, "bar")])
        }

        #[test]
        fn it_splits_target_and_term() {
            let result = parse_match_option(Some("env:bar"));
            assert_eq!(result, vec![(Some("env"), "bar")])
        }

        #[test]
        fn it_multiple() {
            let result = parse_match_option(Some("foo,alias:wat"));
            assert_eq!(result, vec![(None, "foo"), (Some("alias"), "wat")])
        }
    }

    mod filter_match {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_includes_all_values_when_empty() {
            let result = filter_match(&[], &["env"], "foo");
            assert_eq!(result, true)
        }

        #[test]
        fn it_includes_untagged_match() {
            let result = filter_match(&[(None, "foo")], &["env"], "foo");
            assert_eq!(result, true)
        }

        #[test]
        fn it_includes_tagged_matches() {
            let result = filter_match(&[(Some("env"), "foo")], &["env"], "foo");
            assert_eq!(result, true)
        }

        #[test]
        fn it_excludes_non_matching_tagged_non_match() {
            let result = filter_match(&[(Some("not-env"), "foo")], &["env"], "foo");
            assert_eq!(result, false)
        }

        #[test]
        fn it_excludes_untagged_non_match() {
            let result = filter_match(&[(None, "bar")], &["env"], "foo");
            assert_eq!(result, false)
        }
    }

    mod filter_exclude {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_includes_all_values_when_empty() {
            let result = filter_exclude(&[], &["env"], "foo");
            assert_eq!(result, true)
        }

        #[test]
        fn it_excludes_untagged_match() {
            let result = filter_exclude(&[(None, "foo")], &["env"], "foo");
            assert_eq!(result, false)
        }

        #[test]
        fn it_excludes_tagged_matches() {
            let result = filter_exclude(&[(Some("env"), "foo")], &["env"], "foo");
            assert_eq!(result, false)
        }

        #[test]
        fn it_includes_non_matching_tag() {
            let result = filter_exclude(&[(Some("not-env"), "foo")], &["env"], "foo");
            assert_eq!(result, true)
        }

        #[test]
        fn it_includes_untagged_non_match() {
            let result = filter_exclude(&[(None, "bar")], &["env"], "foo");
            assert_eq!(result, true)
        }
    }
}
