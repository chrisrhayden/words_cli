mod cache;
mod dict_api;
mod formatter;
mod test_utils;
mod utils;

use std::error::Error;

use clap::Clap;

use cache::{cache_word, get_from_cache};
use dict_api::{get_definition, RequestOptions};
use formatter::print_definition;

#[allow(dead_code)]
#[derive(Clap)]
#[clap()]
struct WordArgs {
    #[clap(short, long)]
    columns: Option<usize>,
    #[clap(required = true)]
    query: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = WordArgs::parse();

    let cached_query = get_from_cache(&args.query)?;

    if let Some(cached_query) = cached_query {
        print_definition(&cached_query);
    } else {
        let request_opts = RequestOptions::default();

        let word_data = get_definition(request_opts, &args.query)?;

        cache_word(&word_data)?;

        print_definition(&word_data);
    }
    Ok(())
}
