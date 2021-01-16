use glob::Pattern;

pub struct GlobalOptions<'a> {
    pub as_: Option<&'a str>,
    pub globs: Option<Vec<(Option<&'a str>, &'a str)>>,
    pub filters: Option<Vec<(Option<&'a str>, &'a str)>>,
    pub path: Option<&'a str>,
}

impl<'a> GlobalOptions<'a> {
    pub fn new(
        glob: Option<&'a str>,
        filter: Option<&'a str>,
        as_: Option<&'a str>,
        path: Option<&'a str>,
    ) -> Self {
        let globs = parse_match_option(glob);
        let filters = parse_match_option(filter);

        Self {
            as_,
            path,
            globs,
            filters,
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
        v.split(",")
            .map(|raw| {
                let items = raw.splitn(2, ":").collect::<Vec<_>>();
                match &items[..] {
                    &[tag, g] => (Some(tag), g),
                    _ => (None, raw),
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
