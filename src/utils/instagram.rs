extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_instagram_url(query: &str) -> String {
    if query == "ig" {
        let instagram_dotcom = "https://instagram.com";

        instagram_dotcom.to_string()

    // Check if it looks like a instagram profile
    } else {
        // Assume the other match is "ig page"
        let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
        let instagram_url = format!("https://instagram.com/{}", encoded_query);

        instagram_url
    }
}

pub fn construct_instagram_profile_url(profile: &str) -> String {
    format!("https://instagram.com/{}", profile)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_instagram_url() {
        let fake_query = "ig";
        assert_eq!(construct_instagram_url(fake_query), "https://instagram.com");
    }

    #[test]
    fn test_construct_instagram_url_profile() {
        let fake_query = "ig @faraazofficial";
        assert_eq!(
            construct_instagram_url(fake_query),
            "https://instagram.com/faraazofficial"
        );
    }

    #[test]
    fn test_construct_instagram_profile_url() {
        let fake_profile = "faraazofficial";
        assert_eq!(
            construct_instagram_profile_url(fake_profile),
            "https://instagram.com/faraazofficial"
        );
    }
}
