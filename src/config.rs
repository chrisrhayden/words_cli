//! basic user config
//!
//!
//! a user config and all the default settings
//! ```toml
//! [format_config]
//! # `columns` will try and break at the nearest word going backwards, zero will
//! #           ignore formating
//! columns = 0
//! # `indent_by` will the amount of spaces each part will be indented by
//! indent_by = 2
//! # `search_limit` is the amount of characters to search before giving up
//! search_limit = 10,
//! # `synonym_limit` is the amount of synonyms to show
//! synonym_limit = 5
//!
//!
//! # escape codes to style the dictionary output when sent to a terminal
//! [format_style]
//! word = "\x1b[1m"
//! part_of_speech = "\x1b[1m"
//! definition = "\x1b[0m"
//! example_title = "\x1b[4m"
//! example = "\x1b[3m"
//! synonyms_title = "\x1b[4m"
//! synonyms = "\x1b[0m"
//! ```
use std::{error::Error, fs, path::PathBuf};

use serde::Deserialize;
use toml;

use crate::{
    formatter::{FormatterConfig, FormatterStyle},
    utils::get_user_config_path,
};

/// styling when printing to console
///
///
/// default word: "\x1b[1m"
///
/// default part_of_speech: "\x1b[1m"
///
/// default definition: "\x1b[0m"
///
/// default example_title: "\x1b[4m"
///
/// default example: "\x1b[3m"
///
/// default synonyms_title: "\x1b[4m"
///
/// default synonyms: "\x1b[0m"
///
/// default reset: "\x1b[0m"
#[derive(Deserialize, Debug)]
pub struct FormatStyle {
    pub word: Option<String>,
    pub part_of_speech: Option<String>,
    pub definition: Option<String>,
    pub example_title: Option<String>,
    pub example: Option<String>,
    pub synonyms_title: Option<String>,
    pub synonyms: Option<String>,
    // this will more or less be the same i guess
    pub reset: Option<String>,
}

/// format information when printing out the definition
///
///
/// `columns` will try and break at the nearest word going backwards, zero will
///           ignore formating
///
/// default: will try and run `tput to find out`,
///
/// `indent_by` will the amount of spaces each part will be indented by
///
/// default: 2,
///
/// `search_limit` is the amount of characters to search before giving up
///
/// default: 10,
///
/// `synonym_limit` is the amount of synonyms to show
///
/// default: 5,
#[derive(Deserialize, Debug)]
pub struct FormatConfig {
    pub formating: Option<bool>,
    pub columns: Option<usize>,
    pub indent_by: Option<usize>,
    pub search_limit: Option<usize>,
    pub synonym_limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct WordsConfig {
    pub format_config: Option<FormatConfig>,
    pub format_style: Option<FormatStyle>,
}

impl WordsConfig {
    fn resolve_style(&mut self) -> Option<FormatterStyle> {
        if let Some(style) = self.format_style.as_mut() {
            let mut format_style = FormatterStyle::default();

            if let Some(word) = style.word.take() {
                format_style.word = word;
            }

            if let Some(part_of_speech) = style.part_of_speech.take() {
                format_style.part_of_speech = part_of_speech;
            }

            if let Some(definition) = style.definition.take() {
                format_style.definition = definition;
            }

            if let Some(example_title) = style.example_title.take() {
                format_style.example_title = example_title;
            }

            if let Some(example) = style.example.take() {
                format_style.example = example;
            }

            if let Some(synonyms_title) = style.synonyms_title.take() {
                format_style.synonyms_title = synonyms_title;
            }

            if let Some(synonyms) = style.synonyms.take() {
                format_style.synonyms = synonyms;
            }

            if let Some(reset) = style.reset.take() {
                format_style.reset = reset;
            }

            Some(format_style)
        } else {
            None
        }
    }

    fn resolve_formatter_config(&mut self) -> Option<FormatterConfig> {
        if let Some(config) = self.format_config.as_mut() {
            let mut new_config = FormatterConfig::default();

            if let Some(columns) = config.columns {
                new_config.columns = columns;
            }

            if let Some(indent_by) = config.indent_by {
                new_config.indent_by = indent_by;
            }

            if let Some(search_limit) = config.search_limit {
                new_config.search_limit = search_limit;
            }

            if let Some(synonym_limit) = config.synonym_limit {
                new_config.synonym_limit = synonym_limit;
            }

            Some(new_config)
        } else {
            None
        }
    }

    pub fn resolve_config(&mut self) -> FormatterConfig {
        let mut config = if let Some(conf) = self.resolve_formatter_config() {
            conf
        } else {
            FormatterConfig::default()
        };

        if let Some(style) = self.resolve_style() {
            config.format_style = style;
        };

        config
    }
}

pub fn get_user_config(
    optional_path: Option<&String>,
) -> Result<Option<WordsConfig>, Box<dyn Error>> {
    use std::io::Read;

    let config_toml_path = if let Some(optional_path) = optional_path {
        PathBuf::from(optional_path)
    } else {
        let config_path = get_user_config_path()?;

        config_path.join("words_cli.toml")
    };

    if !config_toml_path.exists() {
        return Ok(None);
    }

    let mut user_config_file = fs::File::open(config_toml_path)?;

    let mut buf = String::new();

    user_config_file.read_to_string(&mut buf)?;

    let user_config: WordsConfig = toml::from_str(&buf)?;

    Ok(Some(user_config))
}

#[cfg(test)]
mod test {
    use super::*;

    use std::env;

    use crate::test_utils::TempSetup;

    fn fake_user_config() -> String {
        r#"
            [format_config]
            formating = true
            columns = 0
            indent_by = 2
            search_limit = 10
            synonym_limit = 5

            [format_style]
            word = "fuck"
            part_of_speech = ""
            definition = ""
            example_title = ""
            example = ""
            synonyms_title = ""
            synonyms = ""
            "#
        .to_string()
    }

    #[test]
    fn test_resolve_format_style_with_user_style() {
        let style_config = FormatStyle {
            word: Some("test".to_string()),
            part_of_speech: Some("test".to_string()),
            definition: Some("test".to_string()),
            example_title: Some("test".to_string()),
            example: Some("test".to_string()),
            synonyms_title: Some("test".to_string()),
            synonyms: Some("test".to_string()),
            reset: Some("test".to_string()),
        };

        let mut word_config = WordsConfig {
            format_style: Some(style_config),
            format_config: None,
        };

        let formatter_style = word_config.resolve_style().unwrap();

        assert_eq!(
            &formatter_style.word, "test",
            "did not set config correctly"
        );

        assert_eq!(
            &formatter_style.part_of_speech, "test",
            "did not set config corectly"
        );
        assert_eq!(
            &formatter_style.definition, "test",
            "did not set config corectly"
        );

        assert_eq!(
            &formatter_style.example_title, "test",
            "did not set correctly"
        );
        assert_eq!(&formatter_style.example, "test", "did not set correctly");

        assert_eq!(
            &formatter_style.synonyms_title, "test",
            "did not set correctly"
        );

        assert_eq!(&formatter_style.synonyms, "test", "did not set correctly");

        assert_eq!(
            &formatter_style.reset, "test",
            "did not set config correctly"
        );
    }

    #[test]
    fn test_resolve_format_style_with_out_user_config() {
        let mut word_config = WordsConfig {
            format_style: None,
            format_config: None,
        };

        match word_config.resolve_style() {
            Some(_) => assert!(
                false,
                "should not get config if there is nothign to merge"
            ),
            None => assert!(true),
        }
    }

    #[test]
    fn test_resolve_format_format_config_with_user_config() {
        let format_config_user = FormatConfig {
            formating: Some(true),
            columns: Some(10),
            indent_by: Some(3),
            search_limit: Some(4),
            synonym_limit: Some(8),
        };

        let mut word_config = WordsConfig {
            format_style: None,
            format_config: Some(format_config_user),
        };

        let formatter_config = word_config.resolve_formatter_config().unwrap();

        assert_eq!(
            formatter_config.columns, 10,
            "did not set config correctly"
        );

        assert_eq!(
            formatter_config.indent_by, 3,
            "did not set config correctly"
        );
        assert_eq!(
            formatter_config.search_limit, 4,
            "did not set config correctly"
        );
        assert_eq!(
            formatter_config.synonym_limit, 8,
            "did not set config correctly"
        );
    }

    #[test]
    fn test_resolve_format_format_config_without_user_config() {
        let mut word_config = WordsConfig {
            format_style: None,
            format_config: None,
        };

        let formatter_config = word_config.resolve_formatter_config();

        match formatter_config {
            Some(_) => assert!(
                false,
                "should not get config when user config is not present"
            ),

            None => assert!(true),
        }
    }

    #[test]
    fn test_resolve_config_both_user_config() {
        let style_config = FormatStyle {
            word: Some("test".to_string()),
            part_of_speech: Some("test".to_string()),
            definition: Some("test".to_string()),
            example_title: Some("test".to_string()),
            example: Some("test".to_string()),
            synonyms_title: Some("test".to_string()),
            synonyms: Some("test".to_string()),
            reset: Some("test".to_string()),
        };

        let format_config_user = FormatConfig {
            formating: Some(true),
            columns: Some(10),
            indent_by: Some(3),
            search_limit: Some(4),
            synonym_limit: Some(8),
        };

        let mut word_config = WordsConfig {
            format_style: Some(style_config),
            format_config: Some(format_config_user),
        };

        let formatter_config = word_config.resolve_config();

        assert_eq!(
            formatter_config.columns, 10,
            "did not set config correctly"
        );

        assert_eq!(
            formatter_config.indent_by, 3,
            "did not set config correctly"
        );
        assert_eq!(
            formatter_config.search_limit, 4,
            "did not set config correctly"
        );
        assert_eq!(
            formatter_config.synonym_limit, 8,
            "did not set config correctly"
        );

        let formatter_style = formatter_config.format_style;

        assert_eq!(
            &formatter_style.word, "test",
            "did not set config correctly"
        );

        assert_eq!(
            &formatter_style.part_of_speech, "test",
            "did not set config corectly"
        );
        assert_eq!(
            &formatter_style.definition, "test",
            "did not set config corectly"
        );

        assert_eq!(
            &formatter_style.example_title, "test",
            "did not set correctly"
        );
        assert_eq!(&formatter_style.example, "test", "did not set correctly");

        assert_eq!(
            &formatter_style.synonyms_title, "test",
            "did not set correctly"
        );

        assert_eq!(&formatter_style.synonyms, "test", "did not set correctly");

        assert_eq!(
            &formatter_style.reset, "test",
            "did not set config correctly"
        );
    }

    #[test]
    fn test_resolve_config_only_user_style_config() {
        let style_config = FormatStyle {
            word: Some("test".to_string()),
            part_of_speech: Some("test".to_string()),
            definition: Some("test".to_string()),
            example_title: Some("test".to_string()),
            example: Some("test".to_string()),
            synonyms_title: Some("test".to_string()),
            synonyms: Some("test".to_string()),
            reset: Some("test".to_string()),
        };

        let mut word_config = WordsConfig {
            format_style: Some(style_config),
            format_config: None,
        };

        let formatter_config = word_config.resolve_config();

        let format_style = formatter_config.format_style;

        assert_eq!(&format_style.word, "test", "did not set config correctly");

        assert_eq!(
            &format_style.part_of_speech, "test",
            "did not set config corectly"
        );
        assert_eq!(
            &format_style.definition, "test",
            "did not set config corectly"
        );

        assert_eq!(
            &format_style.example_title, "test",
            "did not set correctly"
        );
        assert_eq!(&format_style.example, "test", "did not set correctly");

        assert_eq!(
            &format_style.synonyms_title, "test",
            "did not set correctly"
        );

        assert_eq!(&format_style.synonyms, "test", "did not set correctly");

        assert_eq!(&format_style.reset, "test", "did not set config correctly");
    }

    #[test]
    fn test_resolve_config_only_user_formatter_config() {
        let format_config_user = FormatConfig {
            formating: Some(true),
            columns: Some(10),
            indent_by: Some(3),
            search_limit: Some(4),
            synonym_limit: Some(8),
        };

        let mut word_config = WordsConfig {
            format_style: None,
            format_config: Some(format_config_user),
        };

        let formatter_config = word_config.resolve_config();

        assert_eq!(
            formatter_config.columns, 10,
            "did not set config correctly"
        );

        assert_eq!(
            formatter_config.indent_by, 3,
            "did not set config correctly"
        );
        assert_eq!(
            formatter_config.search_limit, 4,
            "did not set config correctly"
        );
        assert_eq!(
            formatter_config.synonym_limit, 8,
            "did not set config correctly"
        );
    }

    #[test]
    fn test_get_user_config() {
        use std::io::Write;

        let mut temp = TempSetup::default();
        let root_path = temp.setup();

        env::set_var("HOME", root_path.as_os_str());

        env::set_var("XDG_CONFIG_HOME", root_path.join(".config").as_os_str());

        let config_path = root_path.join(".config").join("words_cli");

        fs::create_dir_all(&config_path).unwrap();

        let toml_file_path = config_path.join("words_cli.toml");

        let mut toml_file = fs::File::create(toml_file_path).unwrap();

        toml_file.write_all(&fake_user_config().as_bytes()).unwrap();

        let user_config = get_user_config(None).unwrap();

        assert!(user_config.is_some(), "did not find the user config");

        let user_config = user_config.as_ref().unwrap();

        assert!(user_config.format_style.is_some(), "didnt get user config");

        assert_eq!(
            user_config
                .format_style
                .as_ref()
                .unwrap()
                .word
                .as_ref()
                .unwrap(),
            "fuck",
            "get_user_config get the wrong data for format_style"
        );
    }

    #[test]
    fn test_get_user_config_given_path() {
        use std::io::Write;

        let mut temp = TempSetup::default();
        let root_path = temp.setup();

        env::set_var("HOME", root_path.as_os_str());

        env::set_var("XDG_CONFIG_HOME", root_path.join(".config").as_os_str());

        let config_path = root_path.join("test");

        fs::create_dir_all(&config_path).unwrap();

        let toml_file_path = config_path.join("words_cli.toml");

        let mut toml_file = fs::File::create(&toml_file_path).unwrap();

        toml_file.write_all(&fake_user_config().as_bytes()).unwrap();

        let toml_string = toml_file_path.to_string_lossy().to_string();

        let user_config = get_user_config(Some(&toml_string)).unwrap();

        assert!(user_config.is_some(), "did not find the user config");

        let user_config = user_config.as_ref().unwrap();

        assert!(user_config.format_style.is_some(), "didnt get user config");

        assert_eq!(
            user_config
                .format_style
                .as_ref()
                .unwrap()
                .word
                .as_ref()
                .unwrap(),
            "fuck",
            "get_user_config get the wrong data for format_style"
        );
    }
}
