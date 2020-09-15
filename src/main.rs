mod cache;
mod dict_api;
mod formatter;
mod spell;
mod utils;

#[cfg(test)]
mod test_utils;

use std::error::Error;

use clap::Clap;

use crate::{
    cache::{cache_definition, get_from_cache},
    dict_api::{get_definition, RequestOptions},
    formatter::{print_definition, FormatConf},
    spell::check_spelling,
    utils::get_tty_cols,
};

#[allow(dead_code)]
#[derive(Clap)]
#[clap(name = "words_cli", about = "a tool for words")]
struct WordArgs {
    /// print word suggestions
    #[clap(short, long, conflicts_with = "define")]
    suggest: Option<String>,
    /// print word definition
    #[clap(short, long, conflicts_with = "suggest")]
    define: Option<String>,
    /// columns to align definition text
    #[clap(short, long)]
    columns: Option<usize>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = WordArgs::parse();

    if let Some(query) = args.suggest {
        if let Some(suggest_list) = check_spelling(&query)? {
            println!("{}", suggest_list.join("\n"));
        }

        Ok(())
    } else if let Some(query) = args.define {
        let word_data = if let Some(cached_query) = get_from_cache(&query)? {
            cached_query
        } else {
            let request_opts = RequestOptions::default();

            let word_data = get_definition(request_opts, &query)?;

            cache_definition(&word_data)?;

            word_data
        };

        let mut format_conf = FormatConf::default();

        format_conf.columns = if let Some(columns) = args.columns {
            columns
        } else {
            get_tty_cols()
        };

        print_definition(&format_conf, &word_data);

        Ok(())
    } else {
        Err(Box::from("we should not be able to get get here"))
    }
}
