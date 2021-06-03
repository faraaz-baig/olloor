extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_youtube_url(query: &str) -> String {
    if query == "yt" {
        let youtube_dotcom = "https://youtube.com";

        youtube_dotcom.to_string()
    } else {
        // Assume the other match is "yt page"
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let youtube_url = format!(
            "https://www.youtube.com/results?search_query={}",
            encoded_query
        );

        youtube_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_youtube_profile_url_with_gh() {
        let fake_query = "yt";
        assert_eq!(construct_youtube_url(fake_query), "https://youtube.com");
    }

    #[test]
    fn test_construct_youtube_profile_url_with_repo_url() {
        let fake_query = "yt trick2g";
        assert_eq!(
            construct_youtube_url(fake_query),
            "https://www.youtube.com/results?search_query=trick2g"
        );
    }
}
