pub mod helpers{
    use std::ops::Add;

    // Helps to trim the image url to get better quality results
    pub fn trim_url(url: &str) -> String{
        let parts: Vec<&str> = url.splitn(2, "_V1").collect();
        if parts.len() > 1 {
            parts[0].to_string().add("_V1")
            // value.to_string()
        } else {
            url.to_string() // Return the original URL if "_V1" is not found
        }
    }
    
    // Helps to extract id from the given url
    pub fn extract_id(url: &str) ->String {
        let parts: Vec<&str> = url.split('/').collect();
        if let Some(id) = parts.iter().find(|&part| part.starts_with("tt")) {
            id.to_string()
        } else {
            "Not Found".to_string()
        }
    }
}