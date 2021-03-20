use log::*;
/// BookInformation a struct type containing 2 variant of Vec<u32> type and 2 of Vec<String> type.
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
    /// Result type returning book information if library not empty or returning library is empty.
    pub fn display_book_information(&self) -> Result<bool, i32> {
        if self.accession_number.is_empty() {
            return Err(0);
        }
        for index in 0..self.accession_number.len() {
            info!("Author Name {}", self.author_name[index]);
            info!("Book Title {}", self.book_title[index]);
            info!("Accession Number {}", self.accession_number[index]);
            info!("Book is issued or not {}", self.book_issued[index])
        }
        Ok(true)
    }
    /// Function add_new_book is adding/inserting a new book in the collection of library.
    ///
    /// #Arguments
    ///
    /// new_book a struct type contains all BookInformation (struct) variants.
    ///
    /// #Return
    ///
    /// String type returning as Added Successfully.
    pub fn add_new_book(&mut self, new_book: BookInformation) -> String {
        self.accession_number.push(new_book.accession_number[0]);
        let take_book_title = new_book.book_title[0].clone();
        self.book_title.push(take_book_title);
        let take_author_name = new_book.author_name[0].clone();
        self.author_name.push(take_author_name);
        self.book_issued.push(new_book.book_issued[0]);
        String::from("Added Successfully")
    }
    /// Function display_book_by_author is displaying book from struct by a particular author.
    ///
    /// #Arguments
    ///
    /// author_name a string type which contain particular author name.
    ///
    /// #Return
    ///
    /// Function is returning a Result type for number of books by particular author or no book.
    pub fn display_book_by_author(&self, author_name: String) -> Result<bool, i32> {
        if !self.author_name.contains(&author_name) {
            warn!("No book available by this author");
            return Err(0);
        }
        for index in 0..self.accession_number.len() {
            if self.author_name[index] == author_name {
                info!("Author Name {}", self.author_name[index]);
                info!("Book Title {}", self.book_title[index]);
                info!("Accession Number {}", self.accession_number[index]);
                info!("Issued Or Not {}", self.book_issued[index]);
            }
        }
        Ok(true)
    }
    /// Function display_book_by_title is displaying the book based on the title of the book.
    ///
    /// #Arguments
    ///
    /// book_title a String type which contain the title of a particular book.
    ///
    /// #Return
    ///
    /// Function is returning a Result type for number of books of a particular title or not available.
    pub fn display_book_by_title(&self, book_title: String) -> Result<i32, String> {
        let mut count = 0;
        if !self.book_title.contains(&book_title) {
            warn!("Book {} is not available", book_title);
            return Err(String::from("This book is not available in Library"));
        }
        for index in 0..self.accession_number.len() {
            if self.book_title[index] == book_title {
                count += 1;
            }
        }
        info!("Number of books by this title are {}", count);
        Ok(count)
    }
    /// Function total_books_in_library is calculating the number of books present at the moment in library.
    ///
    /// #Arguments
    ///
    /// Function with no argument.
    ///
    /// #Return
    ///
    /// Function is returning a Result type for number of books available or library empty.
    pub fn total_books_in_library(&self) -> Result<i32, i32> {
        let mut count = 0;

        if self.accession_number.is_empty() {
            info!("No books.Please provide some");
            return Err(0);
        } else {
            for index in 0..self.accession_number.len() {
                if self.book_issued[index] == 0 {
                    count += 1;
                }
            }
            info!("Total number of books in Library {}", count);
        }
        Ok(count)
    }
    /// Function issue_book is issuing the books if present and if available at the moment in library.
    ///
    /// #Arguments
    ///
    /// book_title a String type is taking the title of a particular book to get issued.
    ///
    /// #Return
    ///
    /// Function is returning a Result type as book is issued or no available for issue.
    pub fn issue_book(&mut self, book_title: String) -> Result<i32, String> {
        let mut available = 0;
        for index in 0..self.accession_number.len() {
            if self.book_issued[index] == 0 && self.book_title[index] == book_title {
                self.book_issued[index] = 1;
                available += 1;
                info!("This book is issued {}", book_title)
            }
        }
        if available == 0 {
            info!("Book is not available");
            return Err(String::from("Book is already with someone else"));
        }
        Ok(available)
    }
}
