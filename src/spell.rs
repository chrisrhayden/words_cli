use std::{env, error::Error};

use ispell::SpellLauncher;

fn parse_args() -> Result<String, Box<dyn Error>> {
    if env::args().len() == 1 {
        return Err(Box::from("pleas give a query"));
    } else if env::args().len() > 2 {
        return Err(Box::from("too many args"));
    }

    let query = env::args().nth(1).expect("cant get arg");

    if query.is_empty() {
        // NOTE: this probably shouldn't happen but idk
        return Err(Box::from("pleas give query"));
    }

    Ok(query)
}

fn main() -> Result<(), Box<dyn Error>> {
    let query = parse_args()?;

    let mut checker = SpellLauncher::new().aspell().launch()?;

    let spell_errs = checker.check(&query)?;

    if spell_errs.is_empty() {
        println!("{} is spelled correctly", query);

        return Ok(());
    }

    for se in spell_errs {
        for suggestion in se.suggestions {
            println!("{}", suggestion);
        }
    }

    Ok(())
}
