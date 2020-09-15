use std::error::Error;

use ispell::SpellLauncher;

pub fn check_spelling(
    query: &str,
) -> Result<Option<Vec<String>>, Box<dyn Error>> {
    let mut checker = SpellLauncher::new().aspell().launch()?;

    let spell_errs = checker.check(&query)?;

    if spell_errs.is_empty() {
        return Ok(None);
    }

    let mut output = Vec::new();

    for se in spell_errs {
        if se.suggestions.is_empty() {
            return Err(Box::from(format!("no suggestion for {}", query)));
        }

        for suggestion in se.suggestions {
            output.push(suggestion);
        }
    }

    Ok(Some(output))
}
