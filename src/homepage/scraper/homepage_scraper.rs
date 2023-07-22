pub mod homepage_scraper {

    //Importing useful crates
    use crate::helper::helper_functions::helpers;
    use crate::homepage::h_struct::homepage_struct::homepage_struct::Movie;

    //Homepage scrapping function for IMDB
    pub fn homepage_scrapping() {
        let response = reqwest::blocking::get("https://www.imdb.com/calendar/?ref_=nv_mv_cal")
            .unwrap()
            .text()
            .unwrap();

        let document = scraper::Html::parse_document(&response);

        let id_selector =
            scraper::Selector::parse("li.ipc-metadata-list-summary-item>a[href]").unwrap();
        let title_selector =
            scraper::Selector::parse("a.ipc-metadata-list-summary-item__t").unwrap();
        let image_src_selector = scraper::Selector::parse("img.ipc-image[src]").unwrap();

        let mut movies: Vec<Movie> = Vec::new();

        //getting titles
        let titles = document
            .select(&title_selector)
            .map(|x| x.inner_html())
            .collect::<Vec<_>>();

        //getting images
        let image_src_values = document
            .select(&image_src_selector)
            .map(|x| helpers::trim_url(x.value().attr("src").unwrap_or("")))
            .collect::<Vec<_>>();

        //getting ids
        let ids = document
            .select(&id_selector)
            .map(|x| helpers::extract_id(x.value().attr("href").unwrap_or("")))
            .collect::<Vec<_>>();

        // println!("{:?}",ids);

        //Movie struct
        for i in 0..=image_src_values.len() - 1 {
            let movie = Movie {
                id: ids[i].to_string(),
                title: titles[i].to_string(),
                image_src: image_src_values[i].to_string(),
            };
            movies.push(movie);
        }

        // Now, you have all the movie data stored in the 'movies' vector of 'Movie' structs
        for (_, movie) in movies.iter().enumerate() {
            println!("Title: {}", movie.title);
            println!("Image Source: {}", movie.image_src);
            println!("id: {}", movie.id);
            println!("\n")
        }

        //writing the content into the html
        // let mut output_file = File::create("output.html").expect("Failed to create output.html");
        // write!(
        //     output_file,
        //     "<!DOCTYPE html>\n<html>\n<head>\n<title>Top 100 Movies</title>\n<link rel=\"stylesheet\" href=\"styles.css\">\n</head>\n<body>\n<h1>Top 100 Movies</h1>\n<div id=\"movies-container\">\n"
        // )
        // .unwrap();
        // for movie in movies {
        //     write!(
        //         output_file,
        //         "<div class=\"movie-card\">\n<img class=\"movie-image\" src=\"{}\">\n<p class=\"movie-title\">{}</p>\n</div>\n",
        //         movie.image_src, movie.title
        //     )
        //     .unwrap();
        // }
        // write!(output_file, "</div>\n</body>\n</html>").unwrap();
    }
}
