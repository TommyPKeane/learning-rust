// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        unimplemented!()
    }

    //fn len(self) -> usize {
    //    unimplemented!()
    //}

    //fn is_empty(self) -> bool {
    //    unimplemented!()
    //}

    //fn add_book(self, book: Book) {
    //    unimplemented!()
    //}

    //fn print_books(self) {
    //    unimplemented!()
    //}

    //fn oldest_book(self) -> Option<&Book> {
    //    unimplemented!()
    //}
}

fn main() {
    // This shows the desired behavior. Uncomment the code below and
    // implement the missing methods. You will need to update the
    // method signatures, including the "self" parameter!
    let library = Library::new();

    //println!("Our library is empty: {}", library.is_empty());
    //
    //library.add_book(Book::new("Lord of the Rings", 1954));
    //library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    //
    //library.print_books();
    //
    //match library.oldest_book() {
    //    Some(book) => println!("My oldest book is {book}"),
    //    None => println!("My library is empty!"),
    //}
    //
    //println!("Our library has {} books", library.len());
}
