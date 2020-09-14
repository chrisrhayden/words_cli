use crate::dict_api::WordData;

pub fn print_definition(word_data: &WordData) {
    println!("{}", word_data.word);

    for meaning in &word_data.meanings {
        println!("{}", meaning.partOfSpeech);
        for definition in &meaning.definitions {
            println!("{}", definition.definition);

            if let Some(ref exam) = definition.example {
                println!("{}", exam);
            }

            if let Some(ref syns) = definition.synonyms {
                println!("{}", syns.join("\n"));
            }
        }
    }
}
