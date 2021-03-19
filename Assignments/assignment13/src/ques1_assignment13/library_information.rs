use log::*;
/// BookInformation a struct type containing 2 variant of Vce<u32> type and 2 of Vec<String> type.
pub struct BookInformation {
    pub accession_number: Vec<u32>,
    pub author_name: Vec<String>,
    pub book_title: Vec<String>,
    pub book_issued: Vec<u32>,
}

/// BookInformation a struct type is implemented here which contain various library functions.
impl BookInformation {
    /// Function display_book_information is displaying all Library Books data using loop.
    ///
    /// #Arguments
    ///
    /// Function with no argument.
    ///
    /// #Return
    ///
    /// no return from function.
    pub fn display_book_information(&self) {
        for index in 0..self.accession_number.len() {
            info!("Author Name {}", self.author_name[index]);
            info!("Book Title {}", self.book_title[index]);
            info!("Accession Number {}", self.accession_number[index]);
            info!("Book is issued or not {}", self.book_issued[index])
        }
    }
    /// Function add_new_book is adding/inserting a new book in the collection of library.
    ///
    /// #Arguments
    ///
    /// new_book a struct type contains all BookInformation (struct) variants.
    ///
    /// #Return
    ///
    /// no return from function.
    pub fn add_new_book(&mut self, new_book: BookInformation) {
        self.accession_number.push(new_book.accession_number[0]);
        let take_book_title = new_book.book_title[0].clone();
        self.book_title.push(take_book_title);
        let take_author_name = new_book.author_name[0].clone();
        self.author_name.push(take_author_name);
        self.book_issued.push(new_book.book_issued[0]);
        info!("New Book Added");
    }
    /// Function display_book_by_author is displaying book from struct by a particular author.
    ///
    /// #Arguments
    ///
    /// author_name a string type which contain particular author name.
    ///
    /// #Return
    ///
    /// no return from function.
    pub fn display_book_by_author(&self, author_name: String) {
        if !self.author_name.contains(&author_name) {
            warn!("No book available by this author");
        }
        for index in 0..self.accession_number.len() {
            if self.author_name[index] == author_name {
                info!("Author Name {}", self.author_name[index]);
                info!("Book Title {}", self.book_title[index]);
                info!("Accession Number {}", self.accession_number[index]);
                info!("Issued Or Not {}", self.book_issued[index]);
            }
        }
    }
    /// Function display_book_by_title is displaying the book based on the title of the book.
    ///
    /// #Arguments
    ///
    /// book_title a String type which contain the title of a particular book.
    ///
    /// #Return
    ///
    /// a i32 type is returning number of books of a particular title.
    pub fn display_book_by_title(&self, book_title: String) -> i32 {
        let mut count = 0;
        if !self.book_title.contains(&book_title) {
            warn!("Book {} is not available", book_title)
        }
        for index in 0..self.accession_number.len() {
            if self.book_title[index] == book_title {
                count += 1;
            }
        }
        info!("Number of books by this title are {}", count);
        count
    }
    /// Function total_books_in_library is calculating the number of books present at the moment in library.
    ///
    /// #Arguments
    ///
    /// Function with no argument.
    ///
    /// #Return
    ///
    /// a i32 type returning the no of books available.
    pub fn total_books_in_library(&self) -> i32 {
        let mut count = 0;

        if self.accession_number.is_empty() {
            info!("No books.Please provide some");
        } else {
            for index in 0..self.accession_number.len() {
                if self.book_issued[index] == 0 {
                    count += 1;
                }
            }
            info!("Total number of books in Library {}", count);
        }
        count
    }
    /// Function issue_book is issuing the books if present and if available at the moment in library.
    ///
    /// #Arguments
    ///
    /// book_title a String type is taking the title of a particular book to get issued.
    ///
    /// #Return
    ///
    /// a i32 type returning whether the particular book is issued or not.
    pub fn issue_book(&mut self, book_title: String) -> i32 {
        let mut available = 0;
        for index in 0..self.accession_number.len() {
            if self.book_issued[index] == 0 && self.book_title[index] == book_title {
                self.book_issued[index] = 1;
                available += 1;
                info!("This book is issued {}", book_title)
            }
        }
        if available == 0 {
            info!("Book is not available")
        }
        available
    }
}
