use std::collections::HashMap;

type QueryParams = HashMap<String, String>;

#[tokio::main]
async fn main() {
    let query_string_prefixed = "?foo=bar&ab=a+b&cd=c%20d";
    let query_string = "foo=bar&ab=a+b&cd=c%20d";
    let query_prefixed: QueryParams =
        serde_urlencoded::from_str(query_string_prefixed).expect("Couldn't parse with prefix");
    let query: QueryParams =
        serde_urlencoded::from_str(query_string).expect("Couldn't parse without prefix");
    eprintln!("Query Prefixed: {:?}", query_prefixed);
    eprintln!("Query: {:?}", query);
    assert_eq!(query, query_prefixed);
}
