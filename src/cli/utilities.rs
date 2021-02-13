use anyhow::{anyhow, Result};
use std::io::Read;
use std::str::FromStr;

/// Parse a single key-value pair
pub fn parse_key_val<T: FromStr>(s: &str) -> Result<(T, T)>
where
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let mut iter = s.splitn(2, '=');
    let first = iter.next();
    let second = iter.next();
    match (first, second) {
        (Some(first), Some(second)) => Ok((first.parse()?, second.parse()?)),
        _ => Err(anyhow!("Invalid KEY=value: no `=` found in `{}`", s)),
    }
}

/// Accept data from stdin
pub fn get_input(values: &[(String, String)]) -> Vec<(String, String)> {
    let mut result = Vec::new();
    result.extend_from_slice(values);

    let mut buffer = String::new();

    if atty::isnt(atty::Stream::Stdin) {
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("Could not read stdin");

        if !buffer.is_empty() {
            if let Some(b) = buffer.strip_suffix("\n") {
                buffer = b.to_string();
            }

            if let Ok(keyval) = parse_key_val(&buffer) {
                result.push(keyval);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    mod parse_key_val {
        use super::super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_works_with_one_equals() {
            let (key, value): (String, String) = parse_key_val("meow=bar").unwrap();
            assert_eq!(key, "meow".to_string());
            assert_eq!(value, "bar".to_string());
        }

        #[test]
        fn it_works_with_2_plus_equals() {
            let (key, value): (String, String) = parse_key_val("meow=bar=bar2=bar3").unwrap();
            assert_eq!(key, "meow".to_string());
            assert_eq!(value, "bar=bar2=bar3".to_string());
        }

        #[test]
        fn it_fails() {
            let result: Result<(String, String)> = parse_key_val("meow");
            let error = result.err().unwrap();
            assert_eq!(
                format!("{}", error),
                "Invalid KEY=value: no `=` found in `meow`".to_string()
            );
        }
    }
}
