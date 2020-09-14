use std::{error::Error, fs, path::PathBuf};

use tempfile::{tempdir, TempDir};

const MONSTER: &'static str = r#"{"word":"monster","phonetics":[{"text":"/ˈmɑnstər/","audio":"https://lex-audio.useremarkable.com/mp3/monster_us_1.mp3"}],"meanings":[{"partOfSpeech":"noun","definitions":[{"definition":"An imaginary creature that is typically large, ugly, and frightening.","example":"She was made into a horrid, ugly monster.","synonyms":["fabulous creature","mythical creature"]}]},{"partOfSpeech":"transitive verb","definitions":[{"definition":"Criticize or reprimand severely.","example":null,"synonyms":["criticize","censure","condemn","castigate","chastise","lambast","pillory","savage","find fault with","fulminate against","abuse"]}]}]}"#;

#[derive(Default)]
pub struct TempSetup {
    pub root: PathBuf,
    pub temp: Option<TempDir>,
}

impl TempSetup {
    pub fn setup(&mut self) -> PathBuf {
        self.temp = Some(tempdir().unwrap());
        self.root = self.temp.as_ref().unwrap().path().to_owned();

        self.root.clone()
    }

    pub fn root_buf(&self) -> PathBuf {
        self.root.clone()
    }

    pub fn make_fake_cache(&self) {
        let cache_path =
            self.root.join(".local").join("share").join("words_cli");
    }
}

impl Drop for TempSetup {
    fn drop(&mut self) {
        if let Some(temp) = self.temp.take() {
            temp.close().expect("cant close file");
        }
    }
}

pub fn fake_word_strings() -> Vec<(String, String)> {
    vec![("monster".to_string(), MONSTER.to_string())]
}
