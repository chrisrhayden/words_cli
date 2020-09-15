use crate::dict_api::WordData;

pub struct FormatStyle {
    pub word: String,
    pub part_of_speech: String,
    pub definition: String,
    pub example_title: String,
    pub example: String,
    pub synonyms_title: String,
    pub synonyms: String,
    pub reset: String,
}

impl Default for FormatStyle {
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

pub struct FormatConf {
    pub columns: usize,
    pub indent_by: usize,
    pub format_style: FormatStyle,
}

impl Default for FormatConf {
    fn default() -> Self {
        Self {
            columns: 0,
            indent_by: 2,
            format_style: Default::default(),
        }
    }
}

fn break_line(
    config: &FormatConf,
    line_break: usize,
    line_style: &str,
    spaces: &str,
    line: &str,
) -> Vec<String> {
    const SEARCH_LIMIT: usize = 10;

    let mut output: Vec<String> = Vec::new();

    let mut end;
    let mut start = 0;

    let line_len = line.len();

    while start < line_len {
        end = start + line_break;

        let n_line = if end > line_len {
            line.get(start..).expect("cant get the rest of the line")
        } else {
            let cur_line = line.get(start..end).expect("out of bounds");

            for (i, c) in cur_line.chars().rev().take(SEARCH_LIMIT).enumerate()
            {
                if c == ' ' {
                    end = end - i;

                    break;
                }
            }

            line.get(start..end)
                .expect("out of bounds, something is wrong")
        };

        let n_line = format!(
            "{}{}{}{}",
            spaces, line_style, n_line, config.format_style.reset
        );

        output.push(n_line);

        start = end;
    }

    output
}

macro_rules! format_line {
    ($config:expr, $style:expr, $spaces:expr, $line:expr) => {
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

fn format_word_data(format_conf: &FormatConf, word_data: &WordData) -> String {
    let mut output: Vec<String> = Vec::new();

    let spaces = " ".repeat(format_conf.indent_by);
    let def_spaces = spaces.repeat(2);
    let example_spaces = spaces.repeat(3);

    let word = format!(
        "{}{}{}",
        format_conf.format_style.word,
        &word_data.word,
        format_conf.format_style.reset,
    );

    output.push(word);

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
                output.push(String::new());

                let example_title = format!(
                    "{}{}example{}",
                    &def_spaces,
                    format_conf.format_style.example_title,
                    format_conf.format_style.reset,
                );

                output.push(example_title);

                let example_lines = format_line!(
                    format_conf,
                    &format_conf.format_style.example,
                    &example_spaces,
                    example
                );

                output.extend(example_lines);
            }

            if let Some(ref syns) = definition.synonyms {
                output.push(String::new());

                let synonyms_title = format!(
                    "{}{}synonyms{}",
                    def_spaces,
                    format_conf.format_style.synonyms_title,
                    format_conf.format_style.reset,
                );

                output.push(synonyms_title);

                let formatted_syns = syns
                    .iter()
                    .map(|s| {
                        format_line!(
                            format_conf,
                            &format_conf.format_style.synonyms,
                            &example_spaces,
                            s
                        )
                    })
                    .flatten();

                output.extend(formatted_syns);
            }

            if definition_len > 1 && i + 1 != definition_len {
                output.push(String::new());
            }
        }

        if meanings_len > 1 && i + 1 != meanings_len {
            output.push(String::new());
        }
    }

    output.join("\n")
}

pub fn print_definition(format_conf: &FormatConf, word_data: &WordData) {
    let word_str = format_word_data(format_conf, word_data);

    println!("{}", word_str);
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test_utils::fake_word_data;

    #[test]
    fn test_format_line_short_line_no_trailing_comma() {
        let mut conf = FormatConf::default();

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
        let mut conf = FormatConf::default();

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
        let mut conf = FormatConf::default();

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
        let mut conf = FormatConf::default();

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

        // TODO: this is bad and should change
        let fake_word_string =
            "\x1b[1mtest\x1b[0m\n  \x1b[1mtest part of speech\x1b[0m\n    \
            \x1b[0mtest definition\x1b[0m\n\n    \x1b[4mexample\x1b[0m\n      \
            \x1b[3mtest example text\x1b[0m";

        let fake_conf = FormatConf::default();

        let word_str = format_word_data(&fake_conf, &fake_word);

        assert_eq!(&word_str, fake_word_string, "did not format correctly");
    }
}
