fn next_language<'a>(languages: &'a [String], term: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == term {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang_one: &'a str, lang_two: &'a str) -> &'a str {
    if lang_one.len() >= lang_two.len() {
        lang_one
    } else {
        lang_two
    }
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let result = next_language(&languages, "go");
    println!("{}", result);

    let last = last_language(&languages);
    println!("{}", last);

    let lang_one = &languages[1];
    let lang_two = &languages[2];

    println!("{:#?}", longest_language(lang_one, lang_two));
}
