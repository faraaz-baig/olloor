extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_google_search_url(query: &str) -> String {
    if query == "go" {
        let google_dotcom = "https://google.com";

        google_dotcom.to_string()
    } else {
        // Assume the other match is "go page"
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let google_url = format!("https://www.google.com/search?q={}", encoded_query);

        google_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url_with_go() {
        let fake_query = "go";
        assert_eq!(
            construct_google_search_url(fake_query),
            "https://google.com"
        );
    }

    #[test]
    fn test_construct_google_search_url_with_search_query() {
        let fake_query = "go trick2g";
        assert_eq!(
            construct_google_search_url(fake_query),
            "https://www.google.com/search?q=trick2g"
        );
    }
}
