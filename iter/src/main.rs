fn print_elements(elements: &[String]) {
    // for element in elements {
    //     // Behind the scenes, for loops build the iter and call `next()` for us
    //     println!("{element}");
    // }

    // for_each is an iterator consumer, it calls `next` automatically
    // map is an adapter, it doesn't call `next`, it adds a step in the processing pipeline instead
    // in other languages, `map` does consume and calls next but here we need to explicitly add a consumer at the end
    // to start iteration
    elements.iter().for_each(|element| println!("{element}"));
}

// Shorten in place, needs to be mutable
fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|element| element.truncate(1));
}

// Return a new vector
// `collect` is a consumer, calls next for us
fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|element| element.to_uppercase())
        .collect::<Vec<String>>()
}

// Moving elements from one vector into another. We need to take ownership of the first vec
fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|element| vec_b.push(element));
}

// Create Vec<String> for each string inside the collection, one element per character
fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|strings| {
            strings
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // // Iterator is a struct, contains a pointer to the data,
    // // the current position and the end
    // let mut iter = colors.iter();
    // // `.next()` returns the current position item and moves the current position to the next item
    // // This is why iter needs to be mut
    // println!("{:#?}", iter.next());
    // println!("{:#?}", iter.next());
    // println!("{:#?}", iter.next());
    // println!("{:#?}", iter.next());

    print_elements(&colors);
    let exploded = explode(&colors);
    println!("{:#?}", exploded);
    // shorten_strings(&mut colors);

    // print_elements(&colors);
    // let uppercase = to_uppercase(&colors);
    // print_elements(&uppercase);

    // let mut colors_b = vec![];
    // move_elements(colors, &mut colors_b);
    // print_elements(&colors); // can't call this because it has been moved to `move_elements`
    // print_elements(&colors_b);
}
