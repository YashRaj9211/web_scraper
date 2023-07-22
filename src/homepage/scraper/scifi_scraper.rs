use reqwest;
use scraper::{Html, Selector};
use std::vec::Vec;

use crate::homepage::h_struct::scifi_struct::scifi_struct::SciFi;

pub fn scifi_scrapping() {
    let url = "https://www.imdb.com/search/keyword/?sort=user_rating,desc&mode=detail&page=1&genres=Sci-Fi&ref_=kw_ref_gnr";

    // Send the HTTP request and get the response
    let response = reqwest::blocking::get(url).expect("Failed to fetch the webpage.");

    // Get the HTML content as text
    let html_content = response.text().expect("Failed to get HTML content.");

    // Parse the HTML content
    let document = Html::parse_document(&html_content);

    // Create selectors to extract specific information
    let movie_selector = Selector::parse(".lister-item.mode-detail").unwrap();

    // Create a vector to store the movie data
    let mut scifi_movies: Vec<SciFi> = Vec::new();

    for element in document.select(&movie_selector) {
        // Extract movie data
        let title_selector = Selector::parse("h3.lister-item-header a").unwrap();
        let title = element.select(&title_selector).next().unwrap().inner_html();

        let year_selector = Selector::parse("h3.lister-item-header .lister-item-year").unwrap();
        let year = element.select(&year_selector).next().unwrap().inner_html();

        let runtime_genre_selector =
            Selector::parse(".text-muted.text-small .runtime").unwrap();
        let runtime_genre = element
            .select(&runtime_genre_selector)
            .next()
            .unwrap()
            .inner_html();

        let rating_selector = Selector::parse(".ratings-imdb-rating strong").unwrap();
        let rating = element
            .select(&rating_selector)
            .next()
            .unwrap()
            .inner_html();

        let director_selector = Selector::parse(".text-muted.text-small a[href*=/name/]").unwrap();
        let directors: Vec<_> = element
            .select(&director_selector)
            .map(|director| director.inner_html())
            .collect();

        let star_selector =
            Selector::parse(".text-muted.text-small a[href*=/name/][href*=/?ref_=kw_li_st_]")
                .unwrap();
        let star = element.select(&star_selector).next().unwrap().inner_html();

        let votes_selector = Selector::parse(".text-muted.text-small [name=nv]").unwrap();
        let votes = element.select(&votes_selector).next().unwrap().inner_html();

        let description_selector = Selector::parse("p:not(.text-muted.text-small)").unwrap();
        let description = element
            .select(&description_selector)
            .next()
            .unwrap()
            .inner_html();

        // Create a Scifi struct and push it to the vector
        let scifi_movie = SciFi {
            title,
            year,
            runtime_genre,
            imdb_rating: rating,
            directors,
            star,
            votes,
            description,
        };
        scifi_movies.push(scifi_movie);
    }

    // Print the movie data
    for (index, movie) in scifi_movies.iter().enumerate() {
        println!("Movie #{}", index + 1);
        println!("Title: {}", movie.title);
        println!("Year: {}", movie.year);
        println!("Runtime and Genre: {}", movie.runtime_genre);
        println!("IMDb Rating: {}", movie.imdb_rating);
        println!("Directors: {:?}", movie.directors);
        println!("Star: {}", movie.star);
        println!("Votes: {}", movie.votes);
        println!("Description: {}\n", movie.description);
    }
}
