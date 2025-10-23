use serde_json::Value;

pub struct Fork {
    pub name: String,
    pub owner_login: String,
}

// Purpose:
// parse the json for relevant fields 
// input:
// @json : json to be parsed for relevant fields
// returns:
// a Fork object containing the relevant fields from json
pub fn parse_fork(json: &Value) -> Option<Fork> {
    Some(Fork {
        name: json["name"].as_str()?.to_string(),
        owner_login: json["owner"]["login"].as_str()?.to_string(),
    })
}

// Purpose:
// parse a json_array and call parse(fork) on each json object and  get relevant fields
// input: 
// @json_array : an array of json to be parsed 
// returns:
// a list of Fork objects containing relevant information from the json
pub fn parse_forks(json_array: Vec<Value>) -> Vec<Fork> {
    json_array
        .iter()
        .filter_map(|json| parse_fork(json))
        .collect()
}