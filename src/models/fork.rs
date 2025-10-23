use serde_json::Value;

pub struct Fork {
    pub name: String,
    pub owner_login: String,
}

pub fn parse_fork(json: &Value) -> Option<Fork> {
    Some(Fork {
        name: json["name"].as_str()?.to_string(),
        owner_login: json["owner"]["login"].as_str()?.to_string(),
    })
}

pub fn parse_forks(json_array: Vec<Value>) -> Vec<Fork> {
    json_array
        .iter()
        .filter_map(|json| parse_fork(json))
        .collect()
}