extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_dribbble_url(query: &str) -> String {
    if query == "db" {
        let dribbble_dotcom = "https://dribbble.com/";

        dribbble_dotcom.to_string()

    // Check if it looks like a dribbble profile
    } else if &query[..4] == "db @" {
        construct_dribbble_profile_url(&query[4..])
    } else {
        // Assume the other match is "db sometext"
        // and search on dribbble
        construct_dribbble_search_url(&query[3..])
    }
}

pub fn construct_dribbble_profile_url(profile: &str) -> String {
    format!("https://dribbble.com/{}", profile)
}

pub fn construct_dribbble_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let dribbble_search_url = format!("https://dribbble.com/search/{}", encoded_query);

    dribbble_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_dribbble_url() {
        let fake_query = "db";
        assert_eq!(construct_dribbble_url(fake_query), "https://dribbble.com/");
    }

    #[test]
    fn test_construct_dribbble_url_query() {
        let fake_query = "db hello world";
        assert_eq!(
            construct_dribbble_url(fake_query),
            "https://dribbble.com/search/hello%20world"
        );
    }

    #[test]
    fn test_construct_dribbble_url_profile() {
        let fake_query = "db @deninpaul";
        assert_eq!(
            construct_dribbble_url(fake_query),
            "https://dribbble.com/deninpaul"
        );
    }

    #[test]
    fn test_construct_dribbble_profile_url() {
        let fake_profile = "deninpaul";
        assert_eq!(
            construct_dribbble_profile_url(fake_profile),
            "https://dribbble.com/deninpaul"
        );
    }

    #[test]
    fn test_construct_dribbble_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_dribbble_search_url(fake_query),
            "https://dribbble.com/search/hello%20world"
        );
    }
}
