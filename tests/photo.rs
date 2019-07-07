extern crate pexels;
mod constant;

#[cfg(test)]
mod tests {
    use super::constant;

    #[test]
    fn photo_search_test() {
        let pexels_api_client = pexels::Pexels::new(constant::TEST_API_KEY.to_owned());
        let photo_search_result = pexels_api_client.photo_search("mountain".to_owned(), 15, 1);
        assert_eq!(
            "https://api.pexels.com/v1/search/?page=2&per_page=15&query=mountain",
            photo_search_result["next_page"].as_str().unwrap()
        );
    }

    #[test]
    fn photo_curated_test() {
        let pexels_api_client = pexels::Pexels::new(constant::TEST_API_KEY.to_owned());
        let curated_photo_result = pexels_api_client.photo_curated(15, 1);
        assert_eq!(
            "https://api.pexels.com/v1/curated/?page=2&per_page=15",
            curated_photo_result["next_page"].as_str().unwrap()
        );
    }

    #[test]
    fn photo_detail_test() {
        let pexels_api_client = pexels::Pexels::new(constant::TEST_API_KEY.to_owned());
        let photo_detail_result = pexels_api_client.photo_detail("2608372".to_owned());
        assert_eq!(
            "Kei Scampa",
            photo_detail_result["photographer"].as_str().unwrap()
        );
        assert_eq!(
            "https://www.pexels.com/photo/woman-inside-vehicle-2608372/",
            photo_detail_result["url"].as_str().unwrap()
        );
    }
}
