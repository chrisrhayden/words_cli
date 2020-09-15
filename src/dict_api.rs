use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

/// how to make the request
pub struct RequestOptions {
    url: String,
    lang: String,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            url: "https://api.dictionaryapi.dev/api/v2/entries".to_string(),
            lang: "en".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Phonetic {
    pub text: String,
    // the audio link
    pub audio: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Definition {
    pub definition: String,
    pub example: Option<String>,
    pub synonyms: Option<Vec<String>>,
}

// we are serializing from json so we need to use snake case
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Meaning {
    pub partOfSpeech: String,
    pub definitions: Vec<Definition>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WordData {
    pub word: String,
    pub phonetics: Vec<Phonetic>,
    pub meanings: Vec<Meaning>,
}

/// get a definition from dictionaryapi
pub fn get_definition(
    request_opts: RequestOptions,
    query: &str,
) -> Result<WordData, Box<dyn Error>> {
    let url = format!("{}/{}/{}", request_opts.url, request_opts.lang, query);

    let resp = reqwest::blocking::get(&url)
        .map_err(|e| Box::<dyn Error>::from(e.to_string()))?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(Box::from("No Definitions Found"));
    }

    let resp_array = serde_json::from_str(&resp.text()?)?;

    let word_data: WordData = match resp_array {
        Value::Array(json_resp) => serde_json::from_value::<WordData>(
            json_resp
                .into_iter()
                .nth(0)
                .expect("no definitions in api response"),
        )
        .expect("cant parse api response in to WordData"),

        _ => panic!("api response structure has changed or is malformed"),
    };

    Ok(word_data)
}
