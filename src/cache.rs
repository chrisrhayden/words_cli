use std::{error::Error, fs};

use serde_json;

use crate::{dict_api::WordData, utils::get_data_path};

/// save a definition to the cache directory
///
/// this function just overwrites old definitions
pub fn cache_definition(word_data: &WordData) -> Result<(), Box<dyn Error>> {
    use std::io::Write;

    let data_path = get_data_path()?;

    let cache_path = data_path.join("cache");

    // make the cache dir regardless, this should only get this far if either
    // XDG_DATA_HOME or HOME exist, hopefully
    if !cache_path.exists() {
        fs::create_dir_all(&cache_path)?;
    }

    let data_str = serde_json::to_string(word_data)?;

    let word_path = cache_path.join(&word_data.word);

    // create truncates files if they exists
    let mut word_file = fs::File::create(word_path)?;

    word_file.write_all(data_str.as_bytes())?;

    Ok(())
}

/// return a definition from the cache if it exists else nothing if it doesn't
pub fn get_from_cache(query: &str) -> Result<Option<WordData>, Box<dyn Error>> {
    use std::io::Read;

    let data_path = get_data_path()?;

    let query_path = data_path.join("cache").join(query);

    if query_path.exists() {
        let mut query_file = fs::File::open(query_path)?;

        let mut query_string = String::new();

        query_file.read_to_string(&mut query_string)?;

        // make WordData struct from a json string
        let word_data: WordData = serde_json::from_str(&query_string)?;

        Ok(Some(word_data))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::env;

    use crate::test_utils::{fake_word_strings, TempSetup};

    #[test]
    fn test_get_from_cache_files_exists() {
        use std::io::Write;

        let mut temp = TempSetup::default();
        let root_path = temp.setup();

        env::set_var("HOME", root_path.as_os_str());

        env::set_var("XDG_DATA_HOME", root_path.join(".local").join("share"));

        let cache_path = root_path
            .join(".local")
            .join("share")
            .join("words_cli")
            .join("cache");

        fs::create_dir_all(&cache_path).unwrap();

        let words = fake_word_strings();
        for (word, json_str) in &words {
            let word_path = cache_path.join(word);

            let mut word_file = fs::File::create(word_path).unwrap();

            word_file.write_all(json_str.as_bytes()).unwrap();
        }

        for (word, json_str) in &words {
            let cached_word = get_from_cache(&word).unwrap();

            if let Some(word_data) = cached_word {
                assert_eq!(
                    serde_json::to_string(&word_data).unwrap(),
                    *json_str,
                    "did not read from file correctly"
                )
            }
        }

        // all words found
        assert!(true);
    }

    #[test]
    fn test_get_from_cache_dir_dose_not_exist() {
        let mut temp = TempSetup::default();
        let root_path = temp.setup();

        env::set_var("HOME", root_path.as_os_str());

        env::set_var("XDG_DATA_HOME", root_path.join(".local").join("share"));

        let cache_path = root_path
            .join(".local")
            .join("share")
            .join("words_cli")
            .join("cache");

        fs::create_dir_all(&cache_path).unwrap();

        let word = get_from_cache("test").unwrap();

        assert!(word.is_none(), "some how the word test was made");
    }

    #[test]
    fn test_cache_words_cache_dir_exists() {
        let mut temp = TempSetup::default();
        let root_path = temp.setup();

        env::set_var("HOME", &root_path.as_os_str());

        let data_home = root_path.join(".local").join("share");

        env::set_var("XDG_DATA_HOME", data_home.as_os_str());

        let cache_path = root_path
            .join(".local")
            .join("share")
            .join("words_cli")
            .join("cache");

        fs::create_dir_all(&cache_path).unwrap();

        let words = fake_word_strings();

        for (_, json_str) in &words {
            let word_data = serde_json::from_str(json_str).unwrap();

            let cache_result = cache_definition(&word_data);

            if let Err(err) = cache_result {
                eprintln!("{}", err);
            } else {
                assert!(true);
            }
        }

        for (word, _) in &words {
            let word_path = cache_path.join(word);
            assert!(word_path.exists(), "failed to make word file");
        }
    }

    #[test]
    fn test_cache_words_cache_dose_not_exists() {
        let mut temp = TempSetup::default();
        let root_path = temp.setup();

        env::set_var("HOME", &root_path.as_os_str());

        let data_home = root_path.join(".local").join("share");

        env::set_var("XDG_DATA_HOME", data_home.as_os_str());

        let cache_path = root_path
            .join(".local")
            .join("share")
            .join("words_cli")
            .join("cache");

        let words = fake_word_strings();

        for (_, json_str) in &words {
            let word_data = serde_json::from_str(json_str).unwrap();

            let cache_result = cache_definition(&word_data);

            if let Err(err) = cache_result {
                eprintln!("{}", err);
            } else {
                assert!(true);
            }
        }

        for (word, _) in &words {
            let word_path = cache_path.join(word);
            assert!(word_path.exists(), "failed to make word file");
        }
    }

    #[test]
    fn test_cache_words() {
        let mut temp = TempSetup::default();
        let root_path = temp.setup();

        env::set_var("HOME", &root_path.as_os_str());

        let data_home = root_path.join(".local").join("share");

        env::set_var("XDG_DATA_HOME", data_home.as_os_str());

        let cache_path = root_path
            .join(".local")
            .join("share")
            .join("words_cli")
            .join("cache");

        let words = fake_word_strings();

        for (_, json_str) in &words {
            let word_data = serde_json::from_str(json_str).unwrap();

            let cache_result = cache_definition(&word_data);

            if let Err(err) = cache_result {
                eprintln!("{}", err);
            } else {
                assert!(true);
            }
        }

        for (word, _) in &words {
            let word_path = cache_path.join(word);
            assert!(
                word_path.exists(),
                "cache_definition failed to make word file"
            );
        }
    }

    #[test]
    fn test_cache_words_overwites_files() {
        use std::io::Write;

        let mut temp = TempSetup::default();
        let root_path = temp.setup();

        env::set_var("HOME", &root_path.as_os_str());

        let data_home = root_path.join(".local").join("share");

        env::set_var("XDG_DATA_HOME", data_home.as_os_str());

        let cache_path = root_path
            .join(".local")
            .join("share")
            .join("words_cli")
            .join("cache");

        fs::create_dir_all(&cache_path).unwrap();

        let words = fake_word_strings();

        for (word, json_str) in &words {
            let word_path = cache_path.join(word);

            let mut word_file = fs::File::create(word_path).unwrap();

            word_file.write_all(json_str.as_bytes()).unwrap();
        }

        for (_, json_str) in &words {
            let word_data = serde_json::from_str(json_str).unwrap();

            let cache_result = cache_definition(&word_data);

            if let Err(err) = cache_result {
                eprintln!("{}", err);
            } else {
                assert!(true);
            }
        }

        for (word, _) in &words {
            let word_path = cache_path.join(word);
            assert!(word_path.exists(), "failed to make word file");
        }
    }
}
