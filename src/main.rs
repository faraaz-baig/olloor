#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in {}", cmd);

    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command {
        "gh" => utils::github::construct_github_url(&cmd),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "go" => utils::google::construct_google_search_url(&cmd),
        "yt" => utils::youtube::construct_youtube_url(&cmd),
        "ig" => utils::instagram::construct_instagram_url(&cmd),
        "fb" => utils::facebook::construct_facebook_url(&cmd),
        "db" => utils::dribbble::construct_dribbble_url(&cmd),
        "hn" => utils::hackernews::construct_hackernews_url(&cmd),
        _ => utils::duckduckgo::construct_duckduckgo_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}
