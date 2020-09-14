use std::collections::HashMap;
use std::error::Error;

use serde::Deserialize;
use serde_json::{self, Value};

use crate::{world::WORLD, WordConf};

const URL: &'static str = "https://api.dictionaryapi.dev/api/v2/entries";

#[derive(Deserialize, Debug)]
struct Phonetic {
    text: String,
    audio: String,
}

#[derive(Deserialize, Debug)]
struct Definition {
    definition: String,
    example: String,
    synonyms: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct Meaning {
    partOfSpeech: String,
    definitions: Vec<Definition>,
}

#[derive(Deserialize, Debug)]
struct WordData {
    word: String,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
}

fn _break_line(line_limit: usize, line: &str) -> Vec<String> {
    let line = line.trim();

    // the limit to search for a space character when braking the string
    let search_limit = 10;

    // the return collection
    let mut new_lines = Vec::new();

    // the ending point to break the current segment
    let mut end;
    // where to start the current segment
    let mut start = 0;

    while start != line.len() {
        let n_line;

        let line_delta = line.len() - start;

        // if the line_limit is longer the delta then it will fit regardless
        if line_limit > line_delta {
            end = start + line_delta;

            // extract the rest of the string
            n_line =
                line.get(start..end).expect("cant get str iter").to_string();
        } else {
            end = start + line_limit;

            // get the last space from the end of the string
            for (i, c) in line
                .get(start..end)
                .expect("iter targets are out of bounds")
                .chars()
                .rev()
                .take(search_limit)
                .enumerate()
            {
                if c == ' ' {
                    // reset the end value to the current index
                    end = start + (line_limit - i);
                    break;
                }
            }

            // extract the new line from the string
            n_line =
                line.get(start..end).expect("cant get str iter").to_string();
        }

        if n_line != " " {
            // add the new line to the return collection
            new_lines.push(n_line);
        }

        // set the next start value the current end value so we start at the
        // right point the next iteration
        start = end;
    }

    new_lines
}

pub fn get_definition(
    _query: &str,
    _conf: WordConf,
) -> Result<(), Box<dyn Error>> {
    // let url = format!("{}/en/world", URL);
    // let text_resp = reqwest::blocking::get(&url)?.text()?;

    // println!("{}", text_resp);

    // let resp_array = serde_json::from_str(&text_resp)?;

    // let word_definition: WordDefinition = match resp_array {
    //     Value::Array(json_resp) => serde_json::from_value(
    //         json_resp
    //             .into_iter()
    //             .nth(1)
    //             .expect("no definitions in api response"),
    //     )?,
    //     _ => panic!("cant parse api data"),
    // };

    let value: WordData = serde_json::from_str(&WORLD)?;

    println!("{:?}", value);

    Ok(())
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_break_line_no_space() {
//         let line = "11111111111111111111111111111111111111111111111111";

//         let line_break = 10;

//         let lines = break_line(line_break, line);

//         assert_eq!(lines.len(), 5, "did not make the right amount of lines");

//         for line in lines {
//             assert_eq!(line.len(), 10, "did not break lines correctly");
//         }
//     }

//     #[test]
//     fn test_break_line_spaces() {
//         let line = "1111111111 1111111111 1111111111 1111111111 1111111111";

//         let line_break = 10;

//         let lines = break_line(line_break, line);

//         assert_eq!(lines.len(), 5, "did not make the right amount of lines");

//         for line in lines {
//             assert_eq!(line, String::from("1111111111"))
//         }
//     }
// }
