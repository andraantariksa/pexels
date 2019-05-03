impl super::Pexels {
    /// # Arguments
    ///
    /// * query - Get videos related to this `query`.
    /// * per_page - Defines the number of results per page (max: 80)
    /// * page - Defines the number of the page
    ///
    /// # Example
    ///
    /// ```ignore
    /// let x = Pexels::new("YOUR_API_KEY".to_string());
    /// x.search_video("mountains".to_string(), 15, 1);
    /// ```
    pub fn search_video(&self, query: String, per_page: u32, page: u32) -> ::serde_json::Value {
        self.get(
            "videos/search",
            Some(
                [
                    ("query", query),
                    ("per_page", per_page.to_string()),
                    ("page", page.to_string()),
                ]
                .to_vec(),
            ),
        )
    }

    /// # Arguments
    ///
    /// * per_page - Defines the number of results per page (max: 80)
    /// * page - Defines the number of the page
    ///
    /// # Example
    ///
    /// ```ignore
    /// let x = Pexels::new("YOUR_API_KEY".to_string());
    /// x.popular_video(15, 1);
    /// ```
    pub fn popular_video(&self, per_page: u32, page: u32) -> ::serde_json::Value {
        self.get(
            "videos/popular",
            Some(
                [
                    ("per_page", per_page.to_string()),
                    ("page", page.to_string()),
                ]
                .to_vec(),
            ),
        )
    }
}
