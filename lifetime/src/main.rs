// The 'a annotation tells the compiler the return time will use the same lifetime as the languages param
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;
    for language in languages {
        if found {
            return language;
        }

        if language.eq(current) {
            found = true;
        }
    }

    languages.last().unwrap()
}

// Lifetime annotation not needed if there's only one reference
// Rust assumes the lifetime of the result is tied to the lifetime of the parameter
fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

// Lifetime of the return reference will be tied to either lang_a or lang_b's lifetime
fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

// Lifetime is how long a binding can be used
fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("swift"),
    ];

    let result = next_language(&languages, "go");

    println!("{result}");

    println!("{:#?}", longest_language(&languages[0], &languages[1]));
}
