use std::collections::HashMap;
type JsonMap = HashMap<String, serde_json::Value>;

fn main() -> Result<(), serde_json::Error> {
    let input = r#"
        {
            "a": "a",
            "b": {"c": "c", "d": "d"},
            "e": 5
        }"#;
    let map: JsonMap = serde_json::from_str(input)?;
    for (key, value) in map.iter() {
        match key.as_str() {
            "a" => assert!(value.is_string()),
            "b" => assert!(value.is_object()),
            "e" => assert!(value.is_number()),
            _ => (),
        }
    };
    Ok(())
}

// use serde_json::{Value, from_str, to_string};
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let body: Value = from_str(r#"
//         {"a": "a", "b": {"c": "c", "d": "d"}, "e": "e"}
//         "#)?;
//     println!("a: {}", string_or(&body["a"], "not a"));
//     println!("bc: {}", string_or(&body["b"]["c"], "not bc"));
//     println!("bd: {}", string_or(&body["b"]["d"], "not bc"));
//     println!("ea: {}", string_or(&body["e"]["a"], "not ea"));
//     println!("fa: {}", string_or(&body["f"]["a"], "not fa"));
//     Ok(())
// }
//
// fn string_or(v: &Value, default: &str) -> String {
//     if v.is_string() {
//         to_string(v).unwrap()
//     } else {
//         default.to_string()
//     }
// }
