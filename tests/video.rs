extern crate pexels;
mod constant;

#[cfg(test)]
mod tests {
    use super::constant;

    #[test]
    fn video_search_test() {
        let pexels_api_client = pexels::Pexels::new(constant::TEST_API_KEY.to_owned());
        let video_search_result = pexels_api_client.video_search("mountain".to_owned(), 15, 1);
        assert_eq!(
            "http://api-videos.pexels.com/search/mountain",
            video_search_result["url"].as_str().unwrap()
        );
    }

    #[test]
    fn video_popular_test() {
        let pexels_api_client = pexels::Pexels::new(constant::TEST_API_KEY.to_owned());
        let video_popular_result = pexels_api_client.video_popular(15, 1);
        assert_eq!(
            "http://api-videos.pexels.com/popular-videos",
            video_popular_result["url"].as_str().unwrap()
        );
    }
}
