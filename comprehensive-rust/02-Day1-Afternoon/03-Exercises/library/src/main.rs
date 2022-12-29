struct Library {
    books: Vec<Book>,
}

#[derive(PartialEq)]
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
        Library {
            books: vec![],
        }
    }

    fn len(&self) -> usize {
        return self.books.len();
    }

    fn is_empty(&self) -> bool {
        return self.books.len() < 1;
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
        return;
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{} ({})", book.title, book.year)
        }
        return;
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut oldest: Option<&Book> = None;
        for book in &self.books {
            if oldest == None {
                oldest = Some(&book);
            } else {
                if book.year < oldest?.year {
                    oldest = Some(&book);
                }
            }
        }
        return oldest;
    }
}

fn main() {
    // This shows the desired behavior. Uncomment the code below and
    // implement the missing methods. You will need to update the
    // method signatures, including the "self" parameter!

    // let library = &mut Library::new(); // Alternative Declaration
    let mut library = Library::new();

    println!("Our library is empty: {}", library.is_empty());

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("My oldest book is {book}"),
        None => println!("My library is empty!"),
    }

    println!("Our library has {} book(s)", library.len());
}
