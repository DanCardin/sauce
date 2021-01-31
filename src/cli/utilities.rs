use std::error::Error;
use std::io::Read;

/// Parse a single key-value pair
pub fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

/// Accept data from stdin
pub fn get_input(values: &[String]) -> Vec<String> {
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
            result.push(buffer);
        }
    }

    result
}
