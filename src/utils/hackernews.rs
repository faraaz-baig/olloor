extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_hackernews_url(query: &str) -> String {
    if query == "hn" {
        let hackernews_dotcom = "https://news.ycombinator.com/";

        hackernews_dotcom.to_string()

    // Check if it looks like a hackernews profile
    } else if &query[..4] == "hn @" {
        construct_hackernews_profile_url(&query[4..])
    } else {
        // Assume the other match is "hn sometext"
        // and search on hackernews
        construct_hackernews_search_url(&query[3..])
    }
}

pub fn construct_hackernews_profile_url(profile: &str) -> String {
    format!("https://news.ycombinator.com/user?id={}", profile)
}

pub fn construct_hackernews_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let hackernews_search_url = format!("https://hn.algolia.com/?q={}", encoded_query);

    hackernews_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_hackernews_url() {
        let fake_query = "hn";
        assert_eq!(
            construct_hackernews_url(fake_query),
            "https://news.ycombinator.com/"
        );
    }

    #[test]
    fn test_construct_hackernews_url_query() {
        let fake_query = "hn hello world";
        assert_eq!(
            construct_hackernews_url(fake_query),
            "https://hn.algolia.com/?q=hello%20world"
        );
    }

    #[test]
    fn test_construct_hackernews_url_profile() {
        let fake_query = "hn @Faraazbaig";
        assert_eq!(
            construct_hackernews_url(fake_query),
            "https://news.ycombinator.com/user?id=Faraazbaig"
        );
    }

    #[test]
    fn test_construct_hackernews_profile_url() {
        let fake_profile = "Faraazbaig";
        assert_eq!(
            construct_hackernews_profile_url(fake_profile),
            "https://news.ycombinator.com/user?id=Faraazbaig"
        );
    }

    #[test]
    fn test_construct_hackernews_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_hackernews_search_url(fake_query),
            "https://hn.algolia.com/?q=hello%20world"
        );
    }
}
