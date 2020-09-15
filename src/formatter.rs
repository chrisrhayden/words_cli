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

        let n_line = format!("{}{}{}", spaces, line_style, n_line);

        output.push(n_line);

        start = end;
    }

    output
}

macro_rules! format_line {
    ($config:expr, $style:expr, $spaces:expr, $line:expr) => {
        if ($spaces.len() + $line.len()) > $config.columns {
            break_line($config.columns - $spaces.len(), $style, $spaces, $line)
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
