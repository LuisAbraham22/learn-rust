use std::{fs, io::Error};
/**
    String -> Want ownership of text or text that grows or shrink. Uses a heap allocation
    &String -> gets mapped to &str
    &str -> read only reference to a string, won't take ownership of text. Points to some text on the heap.
    Can also be used to refer to a portion of the text. One string lives on the heap and the string slice can point to it from the Stack
*/
fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let errors = extract_errors(text.as_str());
    fs::write("errors.txt", errors.join("\n"))?;

    Ok(())
}

fn extract_errors(text: &str) -> Vec<String> {
    let lines = text.split('\n');
    let mut errors = vec![];

    for line in lines {
        if line.starts_with("ERROR") {
            // Take ownership of the underlying string by copying the value from the heap to another reference
            // This requires another heap allocation which is expensive
            errors.push(line.to_string());
        }
    }

    errors
}
