mod cache;
mod dict_api;
mod formatter;
mod utils;

#[cfg(test)]
mod test_utils;

use std::error::Error;

use clap::Clap;

use cache::{cache_definition, get_from_cache};
use dict_api::{get_definition, RequestOptions};
use formatter::{print_definition, FormatConf};
use utils::get_tty_cols;

#[allow(dead_code)]
#[derive(Clap)]
#[clap(name = "words_cli", about = "a tool for words")]
struct WordArgs {
    #[clap(short, long)]
    columns: Option<usize>,
    #[clap(short, long, conflicts_with = "define")]
    suggest: Option<String>,
    #[clap(short, long, conflicts_with = "suggest")]
    define: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = WordArgs::parse();

    if let Some(_query) = args.suggest {
        // TODO:
        println!("suggestions aren't implemented yet");
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
    } else {
        eprintln!("nothing to do");
    }

    Ok(())
}
