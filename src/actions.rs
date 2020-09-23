use std::error::Error;

use crate::{
    cache::{cache_definition, get_from_cache},
    dict_api::{get_definition, RequestOptions},
    formatter::{print_definition, FormatterConfig},
    get_from_stdin,
    spell::check_spelling,
    WordArgs,
};

enum WordActions {
    Definition,
    Suggest,
    Nothing,
}

pub struct WordAction {
    action: WordActions,
    query: String,
}

impl Default for WordAction {
    fn default() -> Self {
        Self {
            action: WordActions::Nothing,
            query: String::new(),
        }
    }
}

impl WordAction {
    pub fn new(word_args: &WordArgs) -> Self {
        let mut word_action = WordAction::default();

        if let Some(query) = word_args.suggest.as_ref() {
            word_action.query.push_str(query.trim());

            word_action.action = WordActions::Suggest;
        } else if let Some(query) = word_args.define.as_ref() {
            word_action.query.push_str(query.trim());

            word_action.action = WordActions::Definition;
        } else {
            panic!("need something to do");
        }

        word_action
    }

    pub fn run(
        &self,
        format_conf: &FormatterConfig,
    ) -> Result<(), Box<dyn Error>> {
        match self.action {
            WordActions::Definition => self.definition(format_conf),
            WordActions::Suggest => self.suggest(format_conf),
            _ => Err(Box::from("nothing to do, this should not happen")),
        }
    }

    fn suggest(&self, config: &FormatterConfig) -> Result<(), Box<dyn Error>> {
        let suggest_list = if self.query == "-" {
            let query_str = get_from_stdin()?;

            check_spelling(query_str.trim())?
        } else {
            check_spelling(&self.query)?
        };

        if config.print {
            if let Some(suggest_list) = suggest_list {
                println!("{}", suggest_list.join("\n"));
            } else {
                println!("{}", self.query);
            }
        }

        Ok(())
    }

    fn definition(
        &self,
        format_conf: &FormatterConfig,
    ) -> Result<(), Box<dyn Error>> {
        let query = if self.query == "-" {
            get_from_stdin()?.trim().to_string()
        } else {
            self.query.to_owned()
        };

        let word_data = if let Some(cached_query) = get_from_cache(&query)? {
            cached_query
        } else {
            let request_opts = RequestOptions::default();

            let word_data = get_definition(request_opts, &query)?;

            cache_definition(&word_data)?;

            word_data
        };

        if format_conf.print {
            print_definition(&format_conf, &word_data);
        }

        Ok(())
    }
}
