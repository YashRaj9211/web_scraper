fn main() {
    homepage::scraper::homepage_scraper::homepage_scraper::homepage_scrapping();
    homepage::scraper::scifi_scraper::scifi_scrapping();
}



//-------------------------------------Initialization--------------------------------
//-------------------------------------All crates definitions--------------------------------

// Crates definitions
// All crates/modules should be defined here
//Every module should contain two different types - scaper and struct
mod homepage {

    pub mod scraper{
        pub mod homepage_scraper;
        pub mod scifi_scraper;
    }

    pub mod h_struct{
        pub mod homepage_struct;
        pub mod scifi_struct;
    }

}

//Helper functions for extracting / manipulating from scapers
mod helper{

    pub mod helper_functions;

}


