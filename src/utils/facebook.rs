extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_facebook_url(query: &str) -> String {
    if query == "fb" {
        let facebook_dotcom = "https://facebook.com";

        facebook_dotcom.to_string()

    // Check if it looks like a facebook profile
    } else if &query[..4] == "fb @" {
        construct_facebook_profile_url(&query[4..])
    } else {
        // Assume the other match is "fb sometext"
        // and search on facebook
        construct_facebook_search_url(&query[3..])
    }
}

pub fn construct_facebook_profile_url(profile: &str) -> String {
    format!("https://facebook.com/{}", profile)
}

pub fn construct_facebook_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let facebook_search_url = format!("https://facebook.com/search?q={}", encoded_query);

    facebook_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_facebook_url() {
        let fake_query = "fb";
        assert_eq!(construct_facebook_url(fake_query), "https://facebook.com");
    }

    #[test]
    fn test_construct_facebook_url_query() {
        let fake_query = "fb hello world";
        assert_eq!(
            construct_facebook_url(fake_query),
            "https://facebook.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_facebook_url_profile() {
        let fake_query = "fb @faraaz0908";
        assert_eq!(
            construct_facebook_url(fake_query),
            "https://facebook.com/faraaz0908"
        );
    }

    #[test]
    fn test_construct_facebook_profile_url() {
        let fake_profile = "faraaz0908";
        assert_eq!(
            construct_facebook_profile_url(fake_profile),
            "https://facebook.com/faraaz0908"
        );
    }

    #[test]
    fn test_construct_facebook_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_facebook_search_url(fake_query),
            "https://facebook.com/search?q=hello%20world"
        );
    }
}
