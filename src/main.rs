mod actions;
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
    actions::WordAction, config::get_user_config, formatter::FormatterConfig,
    utils::get_tty_cols,
};

#[derive(Clap)]
#[clap(name = "words_cli")]
/// a tool for words
///
/// you can specify stdin by giving a `-` as the query
pub struct WordArgs {
    /// dont print output
    #[clap(short, long)]
    pub no_print: bool,
    /// dont format output
    ///
    /// this will just print everything out as one line
    #[clap(short = "F", long)]
    pub no_formatting: bool,
    /// dont print style escape sequences
    #[clap(short = "S", long)]
    pub no_style: bool,
    /// force styling in a pip
    #[clap(short, long)]
    pub force_style: bool,
    /// print word suggestions or the entered word if its already correct
    #[clap(short, long, conflicts_with = "define")]
    pub suggest: Option<String>,
    /// print word definition
    #[clap(short, long, conflicts_with = "suggest")]
    pub define: Option<String>,
    /// columns to align definition text
    ///
    /// this will make the definition text stay within the specified columns
    #[clap(short, long)]
    pub columns: Option<usize>,
    /// supply a config path
    #[clap(short = "C", long)]
    pub config: Option<String>,
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

    let user_config = get_user_config(args.config.as_ref())?;

    let mut config = match user_config {
        None => FormatterConfig::default(),
        Some(mut val) => val.resolve_config(),
    };

    // override default settings with the cli options
    config.print = !args.no_print;

    if args.no_formatting {
        config.formatting = false;

        config.clear_formating();
    } else {
        config.columns = if let Some(columns) = args.columns {
            columns
        } else {
            get_tty_cols()
        };
    }

    if args.no_style || (!args.force_style && atty::isnt(Stream::Stdout)) {
        config.style = false;

        config.clear_style();
    }

    word_action.run(&config)
}
