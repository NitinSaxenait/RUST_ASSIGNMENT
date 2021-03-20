#[cfg(test)]
pub mod test {
    use crate::ques1_assignment13::library_information::BookInformation;

    #[test]
    fn display_book_by_author_success() {
        let library = BookInformation {
            accession_number: vec![0, 1, 2],
            author_name: vec![
                "Gate".to_string(),
                "Jay Shetty".to_string(),
                "Sadguru".to_string(),
                "Vyasa".to_string(),
            ],

            book_title: vec![
                "How to make friends and Influence People".to_string(),
                "Live Like A Monk".to_string(),
                "Death".to_string(),
                "Bhagavad Gita".to_string(),
            ],
            book_issued: vec![0, 1, 0],
        };
        assert_eq!(
            library.display_book_by_author("Jay Shetty".to_string()),
            Ok(true)
        );
    }
    #[test]
    fn display_book_by_author_failure() {
        let library = BookInformation {
            accession_number: vec![0, 1, 2],
            author_name: vec![
                "Gate".to_string(),
                "Jay Shetty".to_string(),
                "Sadguru".to_string(),
                "Vyasa".to_string(),
            ],

            book_title: vec![
                "How to make friends and Influence People".to_string(),
                "Live Like A Monk".to_string(),
                "Death".to_string(),
                "Bhagavad Gita".to_string(),
            ],
            book_issued: vec![0, 1, 0],
        };
        assert_eq!(
            library.display_book_by_author("Rabindra Nath Tagore".to_string()),
            Err(0)
        );
    }
    #[test]
    fn total_books_in_library_success() {
        let library = BookInformation {
            accession_number: vec![0, 1, 2, 3],
            author_name: vec![
                "Gate".to_string(),
                "Jay Shetty".to_string(),
                "Sadguru".to_string(),
                "Vyasas".to_string(),
            ],

            book_title: vec![
                "How to make friends and Influence People".to_string(),
                "Live Like A Monk".to_string(),
                "Death".to_string(),
                "Bhagavad Gita".to_string(),
            ],
            book_issued: vec![0, 1, 0, 0],
        };
        assert_eq!(library.total_books_in_library(), Ok(3));
    }

    #[test]
    fn display_books_info_success() {
        let library = BookInformation {
            accession_number: vec![0, 1, 2, 3],
            author_name: vec![
                "Gate".to_string(),
                "Jay Shetty".to_string(),
                "Sadguru".to_string(),
                "Vyasas".to_string(),
            ],

            book_title: vec![
                "How to make friends and Influence People".to_string(),
                "Live Like A Monk".to_string(),
                "Death".to_string(),
                "Bhagavad Gita".to_string(),
            ],
            book_issued: vec![0, 1, 0, 0],
        };
        assert_eq!(library.display_book_information(), Ok(true));
    }
    #[test]
    fn add_new_book_check() {
        let mut library = BookInformation {
            accession_number: vec![0, 1, 2, 3],
            author_name: vec![
                "Gate".to_string(),
                "Jay Shetty".to_string(),
                "Sadguru".to_string(),
                "Vyasas".to_string(),
            ],

            book_title: vec![
                "How to make friends and Influence People".to_string(),
                "Live Like A Monk".to_string(),
                "Death".to_string(),
                "Bhagavad Gita".to_string(),
            ],
            book_issued: vec![0, 1, 0, 0],
        };
        assert_eq!(
            library.add_new_book(BookInformation {
                accession_number: vec![4],
                author_name: vec!["Vyasa".to_string()],
                book_title: vec!["Mahabharat".to_string()],
                book_issued: vec![1],
            }),
            ("Added Successfully".to_string())
        );
    }
    #[test]
    fn display_book_by_title_success() {
        let library = BookInformation {
            accession_number: vec![0, 1, 2],
            author_name: vec![
                "Gate".to_string(),
                "Jay Shetty".to_string(),
                "Sadguru".to_string(),
                "Vyasa".to_string(),
            ],

            book_title: vec![
                "How to make friends and Influence People".to_string(),
                "Live Like A Monk".to_string(),
                "Death".to_string(),
                "Bhagavad Gita".to_string(),
            ],
            book_issued: vec![0, 1, 0],
        };
        assert_eq!(library.display_book_by_title("Death".to_string()), Ok(1));
    }
    #[test]
    fn display_book_by_title_failure() {
        let library = BookInformation {
            accession_number: vec![0, 1, 2],
            author_name: vec![
                "Gate".to_string(),
                "Jay Shetty".to_string(),
                "Sadguru".to_string(),
                "Vyasa".to_string(),
            ],

            book_title: vec![
                "How to make friends and Influence People".to_string(),
                "Live Like A Monk".to_string(),
                "Death".to_string(),
                "Bhagavad Gita".to_string(),
            ],
            book_issued: vec![0, 1, 0],
        };
        assert_eq!(
            library.display_book_by_title("Life is to Help".to_string()),
            Err(String::from("This book is not available in Library"))
        );
    }
    #[test]
    fn issue_books_success() {
        let mut library = BookInformation {
            accession_number: vec![0, 1, 2, 3],
            author_name: vec![
                "Gate".to_string(),
                "Jay Shetty".to_string(),
                "Sadguru".to_string(),
                "Vyasas".to_string(),
            ],

            book_title: vec![
                "How to make friends and Influence People".to_string(),
                "Live Like A Monk".to_string(),
                "Death".to_string(),
                "Bhagavad Gita".to_string(),
            ],
            book_issued: vec![0, 1, 0, 0],
        };
        assert_eq!(
            library.issue_book("How to make friends and Influence People".to_string()),
            Ok(1)
        );
    }
    #[test]
    fn issue_books_failure() {
        let mut library = BookInformation {
            accession_number: vec![0, 1, 2, 3],
            author_name: vec![
                "Gate".to_string(),
                "Jay Shetty".to_string(),
                "Sadguru".to_string(),
                "Vyasas".to_string(),
            ],

            book_title: vec![
                "How to make friends and Influence People".to_string(),
                "Live Like A Monk".to_string(),
                "Death".to_string(),
                "Bhagavad Gita".to_string(),
            ],
            book_issued: vec![0, 1, 1, 1],
        };
        assert_eq!(
            library.issue_book("Bhagavad Gita".to_string()),
            Err("Book is already with someone else".to_string())
        );
    }
}
