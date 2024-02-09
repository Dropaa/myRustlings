// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

#[test]
fn main() {
    let my_fav_fruits = vec!["banana", "sushi", "avocado", "peach", "j'ai faim sa meeeeeeeeeeere"];

    // Step 1: Create an iterator from the vector
    let mut my_iterable_fav_fruits = my_fav_fruits.iter(); // Using iter() method

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"sushi")); // Step 2: Call next() to get the next element
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach")); // Step 3: Call next() again
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"j'ai faim sa meeeeeeeeeeere"));
    assert_eq!(my_iterable_fav_fruits.next(), None); // Step 4: Call next() until it returns None
}

