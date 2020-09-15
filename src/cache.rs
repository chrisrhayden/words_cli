use std::{env, error::Error, fs, path::PathBuf};

use serde_json;

use crate::dict_api::WordData;

// try and use either XDG_DATA_HOME or HOME else return an error
fn get_data_path() -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(data_home) = env::var("XDG_DATA_HOME") {
        if data_home.is_empty() {
            return Err(Box::from(
                "XDG_DATA_HOME not set correctly and is empty",
            ));
        }

        Ok(PathBuf::from(data_home).join("words_cli"))
    } else if let Ok(val) = env::var("HOME") {
        if val.is_empty() {
            return Err(Box::from("HOME not set correctly and is empty"));
        }

        Ok(PathBuf::from(val)
            .join(".local")
            .join("share")
            .join("words_cli"))
    } else {
        Err(Box::from(
            "environment variables HOME and XDG_DATA_HOME are not set",
        ))
    }
}

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

    use crate::test_utils::{fake_word_strings, TempSetup};

    #[test]
    fn test_get_cache_path() {
        env::set_var("XDG_DATA_HOME", "test");

        let path = get_data_path().unwrap();

        assert_eq!(
            path,
            PathBuf::from("test/words_cli"),
            "did not make path correctly"
        )
    }

    #[test]
    fn test_get_cache_path_no_var_data() {
        env::set_var("XDG_DATA_HOME", "");

        if let Err(err) = get_data_path() {
            assert_eq!(
                err.to_string(),
                "XDG_DATA_HOME not set correctly and is empty",
                "got a different error then expected "
            )
        } else {
            assert!(false, "some how got data in env var")
        }
    }

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
            assert!(word_path.exists(), "failed to make word file");
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
