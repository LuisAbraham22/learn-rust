mod content;

use content::catalog::Catalog;
use content::media::Media;

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let book = Media::Book {
        title: String::from("Crafting Interpreters"),
        author: String::from("Nystrom"),
    };

    let movie = Media::Movie {
        title: String::from("Interstellar"),
        director: String::from("Nolan"),
    };

    let audiobook = Media::AudioBook {
        title: String::from("Atomic Habits"),
    };

    let podcast = Media::Podcast(1);

    let mut catalog = Catalog::new();
    catalog.add(book);
    catalog.add(movie);
    catalog.add(audiobook);
    catalog.add(podcast);

    println!("{:#?}", catalog);

    // Fetching value at index, returns `Option`
    match catalog.get_at_index(1) {
        Some(value) => {
            println!("Fetched item: {:#?}", value);
        }
        None => {
            println!("No value!");
        }
    };

    // Can panic if value is `None`
    // catalog.get_at_index(1).unwrap();

    // Panics with a custom message, useful when you absolutely need the value otherwise crash
    // catalog
    //     .get_at_index(1)
    //     .expect("Expected there to be an item");

    // Returns a default value
    // catalog.get_at_index(1).unwrap_or(&Media::Placeholder);
}
