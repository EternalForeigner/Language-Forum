use std::collections::HashMap;
use url::Url;

pub fn parse_parameters_from_url(
    relative_url: &str,
) -> Result<HashMap<String, String>, url::ParseError> {
    let mut result: HashMap<String, String> = HashMap::new();

    let dummy_base = Url::parse("https://dummy.com")?;
    let absolute_url = dummy_base.join(relative_url)?;

    if let Some(hash_query) = absolute_url.fragment() {
        let hash_params = Url::parse(&format!("{dummy_base}?{hash_query}"))?;
        
        for (key, value) in hash_params.query_pairs() {
            result.insert(key.into_owned(), value.into_owned());
        }
    }

    // Extract parameters from the query string (search parameters)
    for (key, value) in absolute_url.query_pairs() {
        result.insert(key.into_owned(), value.into_owned());
    }

    Ok(result)
}

pub fn strip_parameters_from_url(relative_url: &str) -> Result<String, url::ParseError> {
    let dummy_base = Url::parse("https://dummy.com")?;
    let absolute_url = dummy_base.join(relative_url)?;
    Ok(absolute_url.path().to_string())
}
