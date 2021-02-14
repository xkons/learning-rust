#[derive(PartialEq)]
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

// Implement <Book> == <BookFormat> comparisons
impl PartialEq<BookFormat> for Book {
    fn eq(&self, other: &BookFormat) -> bool {
        self.format == *other
    }
}

// Implement <BookFormat> == <Book> comparisons
impl PartialEq<Book> for BookFormat {
    fn eq(&self, other: &Book) -> bool {
        *self == other.format
    }
}

fn main() {
  let book1 = Book { isbn: 3, format: BookFormat::Paperback };
  println!("book1 is a paperback book: {}", book1 == BookFormat::Paperback);
  println!("book1 is an Ebook: {}", BookFormat::Ebook != book1);
}