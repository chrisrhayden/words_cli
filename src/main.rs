mod cache;
mod dict_api;
mod formatter;

#[cfg(test)]
mod test_utils;

use std::error::Error;

use clap::Clap;

use cache::{cache_definition, get_from_cache};
use dict_api::{get_definition, RequestOptions};
use formatter::print_definition;

#[allow(dead_code)]
#[derive(Clap)]
#[clap(name = "words_cli", about = "a tool for words")]
struct WordArgs {
    #[clap(short, long)]
    columns: Option<usize>,
    #[clap(short, long, conflicts_with = "define")]
    suggest: bool,
    #[clap(short, long, conflicts_with = "suggest")]
    define: bool,
    #[clap(required = true)]
    query: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = WordArgs::parse();

    if args.suggest {
        // TODO:
        println!("suggestions aren't implemented yet");
    } else {
        let cached_query = get_from_cache(&args.query)?;

        let word_data = if let Some(cached_query) = cached_query {
            cached_query
        } else {
            let request_opts = RequestOptions::default();

            let word_data = get_definition(request_opts, &args.query)?;

            cache_definition(&word_data)?;

            word_data
        };

        print_definition(&word_data);
    }

    Ok(())
}
