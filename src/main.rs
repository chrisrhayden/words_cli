mod cache;
mod config;
mod dict_api;
mod formatter;
mod spell;
mod utils;

#[cfg(test)]
mod test_utils;

use std::{error::Error, io};

use clap::Clap;

use atty::Stream;

use crate::{
    cache::{cache_definition, get_from_cache},
    config::get_user_config,
    dict_api::{get_definition, RequestOptions},
    formatter::{print_definition, FormatterConfig},
    spell::check_spelling,
    utils::get_tty_cols,
};

#[derive(Clap)]
#[clap(name = "words_cli")]
/// a tool for words
///
/// NOTE: you can specify stdin by giving a - as the query
struct WordArgs {
    /// dont print output
    #[clap(short, long)]
    no_print: bool,
    /// dont format output
    ///
    /// this will just print everything out as one line
    #[clap(short = "F", long)]
    no_formatting: bool,
    /// dont print style escape sequences
    #[clap(short = "S", long)]
    no_style: bool,
    /// print word suggestions or the entered word if its already correct
    #[clap(short, long, conflicts_with = "define")]
    suggest: Option<String>,
    /// print word definition
    #[clap(short, long, conflicts_with = "suggest")]
    define: Option<String>,
    /// columns to align definition text
    ///
    /// this will make the definition text stay within the specified columns
    #[clap(short, long)]
    columns: Option<usize>,
    /// supply a config path
    #[clap(short = "C", long)]
    config: Option<String>,
}

enum WordActions {
    Definition,
    Suggest,
    Nothing,
}

struct WordAction {
    action: WordActions,
    query: String,
    no_print: bool,
}

impl Default for WordAction {
    fn default() -> Self {
        Self {
            action: WordActions::Nothing,
            query: String::new(),
            no_print: false,
        }
    }
}

impl WordAction {
    fn new(word_args: &WordArgs) -> Self {
        let mut word_action = WordAction::default();

        if let Some(query) = word_args.suggest.as_ref() {
            word_action.query = query.trim().to_string();

            word_action.action = WordActions::Suggest;
        } else if let Some(query) = word_args.define.as_ref() {
            word_action.query = query.trim().to_string();

            word_action.action = WordActions::Definition;
        } else {
            panic!("need something to do");
        }

        word_action.no_print = word_args.no_print;

        word_action
    }

    fn run(&self, args: &WordArgs) -> Result<(), Box<dyn Error>> {
        match self.action {
            WordActions::Definition => self.definition(&args),
            WordActions::Suggest => self.suggest(),
            _ => Err(Box::from("nothing to do, this should not happen")),
        }
    }

    fn suggest(&self) -> Result<(), Box<dyn Error>> {
        let suggest_list = if self.query == "-" {
            let query_str = get_from_stdin()?;

            check_spelling(query_str.trim())?
        } else {
            check_spelling(&self.query)?
        };

        if self.no_print == false {
            if let Some(suggest_list) = suggest_list {
                println!("{}", suggest_list.join("\n"));
            } else {
                println!("{}", self.query);
            }
        }

        Ok(())
    }

    fn definition(&self, args: &WordArgs) -> Result<(), Box<dyn Error>> {
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

        let config = get_user_config(args.config.as_ref())?;

        let mut format_conf = if let Some(mut config) = config {
            config.resolve_config()
        } else {
            FormatterConfig::default()
        };

        if args.no_formatting || !format_conf.formatting {
            format_conf.clear_formating();
        } else {
            format_conf.columns = if let Some(columns) = args.columns {
                columns
            } else {
                get_tty_cols()
            };
        }

        if args.no_style || !format_conf.style || atty::is(Stream::Stdout) {
            format_conf.clear_style();
        }

        if args.no_print == false {
            print_definition(&format_conf, &word_data);
        }

        Ok(())
    }
}

// read input from stdin if asked for
fn get_from_stdin() -> Result<String, Box<dyn Error>> {
    use std::io::Read;

    // get and lock stdin to read safely from stdin
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();

    stdin.read_to_string(&mut buf)?;

    if buf.is_empty() {
        Err(Box::from("nothing in stdin"))
    } else {
        let output = buf.trim().to_string();

        Ok(output)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = WordArgs::parse();

    let word_action = WordAction::new(&args);

    word_action.run(&args)
}
