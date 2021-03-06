use crate::dict_api::WordData;

/// how to style the different part's of the definition
pub struct FormatterStyle {
    pub word: String,
    pub part_of_speech: String,
    pub definition: String,
    pub example_title: String,
    pub example: String,
    pub synonyms_title: String,
    pub synonyms: String,
    pub reset: String,
}

impl Default for FormatterStyle {
    fn default() -> Self {
        Self {
            word: "\x1b[1m".to_string(),
            part_of_speech: "\x1b[1m".to_string(),
            definition: "\x1b[0m".to_string(),
            example_title: "\x1b[4m".to_string(),
            example: "\x1b[3m".to_string(),
            synonyms_title: "\x1b[4m".to_string(),
            synonyms: "\x1b[0m".to_string(),
            reset: "\x1b[0m".to_string(),
        }
    }
}

pub struct FormatterConfig {
    pub print: bool,
    pub style: bool,
    pub formatting: bool,
    pub format_style: FormatterStyle,
    pub columns: usize,
    pub indent_by: usize,
    pub search_limit: usize,
    pub synonym_limit: usize,
}

impl FormatterConfig {
    pub fn clear_style(&mut self) {
        self.style = false;
        self.format_style.word = String::new();
        self.format_style.part_of_speech = String::new();
        self.format_style.definition = String::new();
        self.format_style.example_title = String::new();
        self.format_style.example = String::new();
        self.format_style.synonyms_title = String::new();
        self.format_style.synonyms = String::new();
        self.format_style.reset = String::new();
    }

    pub fn clear_formating(&mut self) {
        self.formatting = false;
        self.columns = 0;
        self.indent_by = 0;
        self.search_limit = 0;
        // this is probably good, idk
        self.synonym_limit = 100;
    }
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            print: true,
            style: true,
            formatting: true,
            columns: 0,
            indent_by: 2,
            format_style: Default::default(),
            search_limit: 10,
            synonym_limit: 5,
        }
    }
}

fn break_line(
    config: &FormatterConfig,
    line_break: usize,
    line_style: &str,
    spaces: &str,
    line: &str,
) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    // end of the segment
    let mut end;
    // begging of new segment
    let mut start = 0;

    let line_len = line.len();

    // if start gets over line_break then the we have prosed the end if the line
    while start < line_len {
        end = start + line_break;

        // if end is greater then line break then we are at the end segment, the
        // next `start = end` will make sure we break the loop
        let n_line = if end > line_len {
            line.get(start..).expect("cant get the rest of the line")
        } else {
            // grab a segment from the string to search
            let cur_line = line.get(start..end).expect("out of bounds");

            // the search_iter
            let back_line_search =
                cur_line.chars().rev().take(config.search_limit).enumerate();

            for (i, c) in back_line_search {
                if c == ' ' {
                    // move back the amount of characters before the first space
                    end = end - i;

                    // we found a space so end the loop
                    break;
                }
            }

            // get the real segment
            line.get(start..end)
                .expect("out of bounds, something is wrong")
        };

        let n_line = format!(
            "{}{}{}{}",
            spaces, line_style, n_line, config.format_style.reset
        );

        output.push(n_line);

        // now start the next iteration at the current ending point
        start = end;
    }

    output
}

macro_rules! format_line {
    ($config:expr, $style:expr, $spaces:expr, $line:expr) => {
        // if columns is zero or the line with the given spaces is too long to
        // fit in the columns
        if $config.columns != 0
            && ($spaces.len() + $line.len()) > $config.columns
        {
            break_line(
                $config,
                $config.columns - $spaces.len(),
                $style,
                $spaces,
                $line,
            )
        } else {
            // NOTE: this returns a vec even if there is only one line, there
            // probably is a better way
            vec![format!(
                "{}{}{}{}",
                $spaces, $style, $line, $config.format_style.reset
            )]
        }
    };
    ($style:expr, $reset:expr, $spaces:expr, $line:expr,) => {
        format_line!($style, $reset, $spaces, $line)
    };
}

fn format_word_data(
    format_conf: &FormatterConfig,
    word_data: &WordData,
) -> String {
    // a vec to collect the lines
    let mut output: Vec<String> = Vec::new();

    // setup the indent levels
    let spaces = " ".repeat(format_conf.indent_by);
    let def_spaces = spaces.repeat(2);
    let exa_spaces = spaces.repeat(3);

    // format the queried word
    let word = format_line!(
        format_conf,
        &format_conf.format_style.word,
        "",
        &word_data.word,
    );

    output.extend(word);

    let meanings_len = word_data.meanings.len();

    for (i, meaning) in word_data.meanings.iter().enumerate() {
        let part_of_speech_titles = format_line!(
            format_conf,
            &format_conf.format_style.part_of_speech,
            &spaces,
            &meaning.partOfSpeech,
        );

        output.extend(part_of_speech_titles);

        let definition_len = meaning.definitions.len();

        for (i, definition) in meaning.definitions.iter().enumerate() {
            let definition_lines = format_line!(
                &format_conf,
                &format_conf.format_style.definition,
                &def_spaces,
                &definition.definition,
            );

            output.extend(definition_lines);

            if let Some(ref example) = definition.example {
                if format_conf.formatting {
                    // add an empty string to output to add a new line in the
                    // formated output
                    output.push(String::new());
                }

                let example_title = format_line!(
                    format_conf,
                    &format_conf.format_style.example_title,
                    &def_spaces,
                    "example",
                );

                output.extend(example_title);

                let example_lines = format_line!(
                    format_conf,
                    &format_conf.format_style.example,
                    &exa_spaces,
                    example
                );

                output.extend(example_lines);
            }

            // NOTE: if expr && if let ... is nightly / experimental i guess
            if format_conf.synonym_limit > 0 {
                if let Some(ref syns) = definition.synonyms {
                    if format_conf.formatting {
                        output.push(String::new());
                    }

                    let synonyms_title = format!(
                        "{}{}synonyms{}",
                        def_spaces,
                        format_conf.format_style.synonyms_title,
                        format_conf.format_style.reset,
                    );

                    output.push(synonyms_title);

                    let formatted_syns = syns
                        .iter()
                        .take(format_conf.synonym_limit)
                        .map(|s| {
                            format_line!(
                                format_conf,
                                &format_conf.format_style.synonyms,
                                &exa_spaces,
                                s
                            )
                        })
                        .flatten();

                    output.extend(formatted_syns);
                }
            }

            if format_conf.formatting
                && definition_len > 1
                && i + 1 != definition_len
            {
                output.push(String::new());
            }
        }

        if format_conf.formatting && meanings_len > 1 && i + 1 != meanings_len {
            output.push(String::new());
        }
    }

    if format_conf.formatting {
        output.join("\n")
    } else {
        output.join(" ")
    }
}

pub fn print_definition(format_conf: &FormatterConfig, word_data: &WordData) {
    let word_str = format_word_data(format_conf, word_data);

    println!("{}", word_str);
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils::fake_word_data;

    fn make_formatted_text_one() -> String {
        "\x1b[1mtest\x1b[0m\n  \x1b[1mtest part of speech\x1b[0m\n    \
            \x1b[0mtest definition\x1b[0m\n\n    \x1b[4mexample\x1b[0m\n      \
            \x1b[3mtest example text\x1b[0m"
            .to_string()
    }

    fn make_formatted_text_two() -> String {
        let mut fake_word_string_two =
            "\n\n    \x1b[4msynonyms\x1b[0m\n".to_string();

        let test_syn = "      \x1b[0mtest\x1b[0m";

        let fake_word_string_two_p2 = format!("{}\n", test_syn).repeat(4);

        fake_word_string_two.push_str(&fake_word_string_two_p2);
        fake_word_string_two.push_str(test_syn);

        fake_word_string_two
    }

    fn make_fake_word_text_no_formatting() -> String {
        let mut fake_word_string_one =
            "test test part of speech test definition \
            example test example text synonyms"
                .to_string();

        let test_syns = " test".repeat(10);

        fake_word_string_one.push_str(&test_syns);

        fake_word_string_one.to_string()
    }

    #[test]
    fn test_format_line_short_line_no_trailing_comma() {
        let mut conf = FormatterConfig::default();

        let line_break = 21;

        conf.columns = line_break;

        let style = "";
        let spaces = "";
        let line = "this is a short line";

        let lines = format_line!(&conf, style, spaces, line);

        assert_eq!(lines.len(), 1, "broke a short line when it shouldn't");

        assert_eq!(
            lines.first().unwrap(),
            "this is a short line\u{1b}[0m",
            "line mangled"
        );
    }

    #[test]
    fn test_format_line_short_line_trailing_comma() {
        let mut conf = FormatterConfig::default();

        let line_break = 20;

        conf.columns = line_break;

        let style = "";
        let spaces = "";
        let line = "this is a short line";

        let lines = format_line!(&conf, style, spaces, line,);

        assert_eq!(lines.len(), 1, "broke a short line when it shouldn't");

        assert_eq!(
            lines.first().unwrap(),
            "this is a short line\u{1b}[0m",
            "line mangled"
        );
    }

    #[test]
    fn test_format_line_long_line_with_spaces() {
        let mut conf = FormatterConfig::default();

        let line_break = 20;

        conf.columns = line_break;

        let style = "";
        let spaces = "";
        let line = "this is a vary long lone that will need a lot of word and \
                    words and more words to be very long if i really want this \
                    string to be very long i guess im glad im i dont need it \
                    to be very unique";

        let lines = format_line!(&conf, style, spaces, line,);

        assert_eq!(
            lines.len(),
            11,
            "broke a long in to wrong amount of segments"
        );

        let broken_lines: Vec<&'static str> = vec![
            "this is a vary long \x1b[0m",
            "lone that will need \x1b[0m",
            "a lot of word and \x1b[0m",
            "words and more \x1b[0m",
            "words to be very \x1b[0m",
            "long if i really \x1b[0m",
            "want this string to \x1b[0m",
            "be very long i \x1b[0m",
            "guess im glad im i \x1b[0m",
            "dont need it to be \x1b[0m",
            "very unique\x1b[0m",
        ];

        assert_eq!(lines, broken_lines, "broke lines in the wrong way")
    }

    #[test]
    fn test_format_line_long_line_without_spaces() {
        let mut conf = FormatterConfig::default();

        let line_break = 20;

        conf.columns = line_break;

        let style = "";
        let spaces = "";

        let line = "11111111111111111111111111111111111111111111111111\
                    11111111111111111111111111111111111111111111111111";

        let lines = format_line!(&conf, style, spaces, line,);

        assert_eq!(
            lines.len(),
            5,
            "broke a long in to wrong amount of segments"
        );

        let broken_lines: Vec<&'static str> = vec![
            "11111111111111111111\x1b[0m",
            "11111111111111111111\x1b[0m",
            "11111111111111111111\x1b[0m",
            "11111111111111111111\x1b[0m",
            "11111111111111111111\x1b[0m",
        ];

        assert_eq!(lines, broken_lines, "broke lines in the wrong way")
    }

    #[test]
    fn test_format_word_data() {
        let fake_word = fake_word_data();

        let mut fake_word_string = make_formatted_text_one();
        fake_word_string.push_str(&make_formatted_text_two());

        let fake_conf = FormatterConfig::default();

        let word_string = format_word_data(&fake_conf, &fake_word);

        assert_eq!(word_string, fake_word_string, "did not format correctly");
    }

    #[test]
    fn test_format_word_data_synonyms_are_truncated() {
        let fake_word = fake_word_data();

        // TODO: this is bad and should change
        let mut fake_word_string = make_formatted_text_one();

        fake_word_string.push_str(&make_formatted_text_two());

        let fake_conf = FormatterConfig::default();

        let word_str = format_word_data(&fake_conf, &fake_word);

        assert_eq!(&word_str, &fake_word_string, "did not format correctly");
    }

    #[test]
    fn test_format_word_data_no_synonyms() {
        let fake_word = fake_word_data();

        // TODO: this is bad and should change
        let fake_word_string = make_formatted_text_one();

        let mut fake_conf = FormatterConfig::default();

        fake_conf.synonym_limit = 0;

        let word_str = format_word_data(&fake_conf, &fake_word);

        assert_eq!(&word_str, &fake_word_string, "did not format correctly");
    }

    #[test]
    fn test_clear_formating_and_style() {
        let mut formatter_config = FormatterConfig::default();

        formatter_config.clear_formating();

        assert_eq!(formatter_config.columns, 0, "did not clear columns");
        assert_eq!(formatter_config.indent_by, 0, "didn not clear indent_by");
        assert_eq!(
            formatter_config.search_limit, 0,
            "did not clear search_limit"
        );
        assert_eq!(
            formatter_config.synonym_limit, 100,
            "didn not clear synonym_limit"
        );

        formatter_config.clear_style();

        assert!(
            formatter_config.format_style.word.is_empty(),
            "did not clear word"
        );
        assert!(
            formatter_config.format_style.part_of_speech.is_empty(),
            "did not clear part_of_speech"
        );
        assert!(
            formatter_config.format_style.definition.is_empty(),
            "did not clear definition"
        );
        assert!(
            formatter_config.format_style.example_title.is_empty(),
            "did not clear example_title"
        );
        assert!(
            formatter_config.format_style.example.is_empty(),
            "did not clear example"
        );
        assert!(
            formatter_config.format_style.synonyms_title.is_empty(),
            "did not clear synonyms_title"
        );
        assert!(
            formatter_config.format_style.synonyms.is_empty(),
            "did not clear synonyms"
        );
        // this will more or less be the same i guess
        assert!(
            formatter_config.format_style.reset.is_empty(),
            "did not clear reset"
        );
    }

    // doing both formatting and style makes checking the string easier
    #[test]
    fn test_format_word_data_no_formating_and_style() {
        let fake_word = fake_word_data();

        let fake_word_string = make_fake_word_text_no_formatting();

        let mut fake_conf = FormatterConfig::default();

        fake_conf.formatting = false;

        fake_conf.clear_formating();
        fake_conf.clear_style();

        let word_string = format_word_data(&fake_conf, &fake_word);

        assert_eq!(word_string, fake_word_string, "did not format correctly");
    }
}
