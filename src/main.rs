mod dict;
mod utils;
mod world;

use std::error::Error;

use clap::Clap;

use dict::get_definition;
use utils::get_tty_cols;

#[derive(Clap)]
#[clap()]
struct WordArgs {
    #[clap(short, long)]
    columns: Option<usize>,
    #[clap(required = true)]
    query: String,
}

pub struct WordConf {
    line_limit: usize,
}

// fn parse_args() -> Result<Option<usize>, Box<dyn Error>> {
//     if env::args().len() == 1 {
//         return Ok(None);
//     } else if env::args().len() > 2 {
//         return Err(Box::from("too many args"));
//     }

//     let cols = env::args().nth(1).expect("cant get arg");

//     if cols.is_empty() {
//         // NOTE: this probably shouldn't happen but idk
//         return Err(Box::from("pleas give query"));
//     }

//     let cols = cols.trim().parse::<usize>().unwrap();

//     Ok(Some(cols))
// }

fn main() -> Result<(), Box<dyn Error>> {
    let args = WordArgs::parse();

    let cols = if let Some(cols) = args.columns {
        cols
    } else {
        get_tty_cols()
    };

    let conf = WordConf { line_limit: cols };

    get_definition(&args.query, conf)
}
