use std::fs::File;
use std::io::prelude::*;

// Animals recognized by the program
#[derive(Debug, PartialEq)]
enum Animal {
    Dog,
    Cat,
    Jellyfish,
}

// Possible errors when reading file
#[derive(Debug)]
enum ReadFileError {
    CannotReadFile,
    WrongFileContents,
}

// Map text to animals. Unknown text is mapped to nothing
// and empty text is mapped to an error.
fn get_animal(text: &str) -> Option<Animal> {
    match text.trim().to_lowercase().as_ref() {
        "dog" => Some(Animal::Dog),
        "cat" => Some(Animal::Cat),
        "jellyfish" => Some(Animal::Jellyfish),
        _ => None,
    }
}

// Read a file to an owned string
fn read_file(file_name: &str) -> Result<String, ReadFileError> {
    use ReadFileError::*;
    let mut file = File::open(file_name).map_err(|_| CannotReadFile)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .map_err(|_| WrongFileContents)?;
    let first_line = file_contents.lines().next().ok_or(WrongFileContents)?;
    Ok(first_line.to_owned())
}

// Entry point to the program reads a file called animals.txt from the
// current working directory and displays a representing sound for the
// animal if it's recognized by the program.
fn main() -> Result<(), ReadFileError> {
    let animal = read_file("animal.txt")?;
    if let Some(result) = get_animal(&animal) {
        let sound = match result {
            Animal::Dog => "woof",
            Animal::Cat => "meow",
            _ => "silence",
        };
        println!("I heard: {}!", sound);
    } else {
        println!("I heard nothing");
    }
    Ok(())
}

// Unit tests for get_animal function. It could also be possible to
// unit test the read_file function, but that would require temporary
// files and would be out of scope for this demonstration.
#[test]
fn test_get_animals() {
    assert_eq!(get_animal("dog"), Some(Animal::Dog));
    assert_eq!(get_animal("Dog"), Some(Animal::Dog));
    assert_eq!(get_animal("cat"), Some(Animal::Cat));
    assert_eq!(get_animal("cAT"), Some(Animal::Cat));
    assert_eq!(get_animal("jellyfish"), Some(Animal::Jellyfish));
    assert_eq!(get_animal("something else"), None);
    assert_eq!(get_animal("-1"), None);
    assert_eq!(get_animal("🐕"), None);
}
