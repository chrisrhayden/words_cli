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

// TODO: this will break if when spelling source changes and when the spell
// checkers change what they return
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_spelling_bad_word() {
        let query = "flgrent";

        let fake_suggestions = vec![
            "flagrant",
            "fragrant",
            "flagrancy",
            "flagrantly",
            "filigreed",
            "flagrance",
            "filigreeing",
            "flagellant",
            "belligerent",
        ];

        if let Some(suggestions) = check_spelling(query).unwrap() {
            for sug in suggestions {
                if !fake_suggestions.contains(&sug.as_ref()) {
                    assert!(false, "got a weird suggestion")
                }
            }
        }
    }
}
