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
            example: "\x1b[0m".to_string(),
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

fn format_definition(format_conf: &FormatConf, word_data: &WordData) -> String {
    let mut output: Vec<String> = Vec::new();

    let spaces = " ".repeat(format_conf.indent_by);

    let reset = &format_conf.format_style.reset;

    let word = format!(
        "{}{}{}",
        format_conf.format_style.word, word_data.word, reset
    );

    output.push(word);

    let meanings_len = word_data.meanings.len();

    for meaning in &word_data.meanings {
        let part_of = format!(
            "{}{}{}{}",
            spaces,
            format_conf.format_style.part_of_speech,
            meaning.partOfSpeech,
            reset
        );

        output.push(part_of);

        let definition_len = meaning.definitions.len();

        for definition in &meaning.definitions {
            let def_string = format!(
                "{}{}{}{}{}",
                spaces,
                spaces,
                format_conf.format_style.definition,
                definition.definition,
                reset
            );

            output.push(def_string);

            if let Some(ref example) = definition.example {
                output.push(String::new());

                let example_title = format!(
                    "{}{}{}example{}",
                    spaces,
                    spaces,
                    format_conf.format_style.example_title,
                    reset
                );

                output.push(example_title);

                let example_str = format!(
                    "{}{}{}{}{}{}",
                    spaces,
                    spaces,
                    spaces,
                    format_conf.format_style.example,
                    example,
                    reset
                );

                output.push(example_str);
            }

            if let Some(ref syns) = definition.synonyms {
                output.push(String::new());
                let synonyms_title = format!(
                    "{}{}{}synonyms{}",
                    spaces,
                    spaces,
                    format_conf.format_style.synonyms_title,
                    reset
                );
                output.push(synonyms_title);

                for syn in syns {
                    let syn_str = format!(
                        "{}{}{}{}{}{}",
                        spaces,
                        spaces,
                        spaces,
                        format_conf.format_style.synonyms,
                        syn,
                        reset
                    );
                    output.push(syn_str);
                }
            }

            if definition_len > 1 {
                output.push(String::new());
            }
        }

        if meanings_len > 1 {
            output.push(String::new());
        }
    }

    output.join("\n")
}

pub fn print_definition(format_conf: &FormatConf, word_data: &WordData) {
    let word_str = format_definition(format_conf, word_data);

    println!("{}", word_str);
}
