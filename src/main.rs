extern crate serde_json;
use eyre::{Result, WrapErr};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub npd_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precedence: Option<i32>,
    pub os_name: String,
}

fn main() -> Result<()> {
    let input1 = r#" {"npdId": "abc", "osName": "def"} "#;
    let r1: Request = serde_json::from_str(input1).wrap_err("Parse of input 1 failed")?;
    println!("r1 precedence is {:?}", r1.precedence);
    println!("r1 is {:?}", r1);
    println!("r1 serialized is '{}'", serde_json::to_string(&r1).wrap_err("Can't serialize r1")?);
    let input2 = r#" {"npdId": "abc", "precedence": 80, "osName": "def"} "#;
    let r2: Request = serde_json::from_str(input2).wrap_err("Parse of input 2 failed")?;
    println!("r2 precedence is {:?}", r2.precedence);
    println!("r2 is {:?}", r2);
    println!("r2 serialized is '{}'", serde_json::to_string(&r2).wrap_err("Can't serialize r1")?);
    Ok(())
}
