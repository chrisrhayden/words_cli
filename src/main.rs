mod cache;
mod dict_api;
mod formatter;
mod spell;
mod utils;

#[cfg(test)]
mod test_utils;

use std::{error::Error, io};

use clap::Clap;

use crate::{
    cache::{cache_definition, get_from_cache},
    dict_api::{get_definition, RequestOptions},
    formatter::{print_definition, FormatConf},
    spell::check_spelling,
    utils::get_tty_cols,
};

#[derive(Clap)]
#[clap(name = "words_cli")]
/// a tool for words
///
/// NOTE: you can specify stdin by giving a -
struct WordArgs {
    /// print word suggestions
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

    if let Some(mut query) = args.suggest {
        if query == "-" {
            query = get_from_stdin()?;
        }

        let query_str = query.trim();

        if let Some(suggest_list) = check_spelling(query_str)? {
            println!("{}", suggest_list.join("\n"));
        }
    } else if let Some(mut query) = args.define {
        if query == "-" {
            query = get_from_stdin()?;
        }

        let query_str = query.trim();

        let word_data = if let Some(cached_query) = get_from_cache(query_str)? {
            cached_query
        } else {
            let request_opts = RequestOptions::default();

            let word_data = get_definition(request_opts, query_str)?;

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
    }

    Ok(())
}
