use std::collections::HashMap;

use url::Url;

fn main() {
    let uri = r#"https://example.com/products?page="2"&sort=desc&bool=true"#;
    let pairs: HashMap<String, String> = Url::parse(uri)
        .expect("Bad deactivation query string")
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    assert_eq!(pairs["page"], r#""2""#);
    assert_eq!(pairs["sort"], "desc");
    assert_eq!(pairs["bool"], "true");
}
