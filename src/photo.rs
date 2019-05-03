impl super::Pexels {
    /// # Arguments
    ///
    /// * query - Search photos related to the `query`
    /// * per_page - Defines the number of results per page (max: 80)
    /// * page - Defines the number of the page
    ///
    /// # Example
    ///
    /// ```ignore
    /// let x = Pexels::new("YOUR_API_KEY".to_string());
    /// x.search_photo("mountains".to_string(), 15, 1);
    /// ```
    pub fn search_photo(&self, query: String, per_page: u32, page: u32) -> ::serde_json::Value {
        self.get(
            "v1/search",
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

    /// Pexels add at least one new photo per hour to our curated list so that you get a changing selection of trending photos.
    ///
    /// # Arguments
    ///
    /// * per_page - Defines the number of results per page (max: 80)
    /// * page - Defines the number of the page
    ///
    /// # Example
    ///
    /// ```ignore
    /// let x = Pexels::new("YOUR_API_KEY".to_string());
    /// x.curated_photo(15, 1);
    /// ```
    pub fn curated_photo(&self, per_page: u32, page: u32) -> ::serde_json::Value {
        self.get(
            "v1/curated",
            Some(
                [
                    ("per_page", per_page.to_string()),
                    ("page", page.to_string()),
                ]
                .to_vec(),
            ),
        )
    }

    /// If you have the id of a photo, you can retrieve the data of it.
    ///
    /// # Arguments
    ///
    /// * id - id of the photo
    ///
    /// # Example
    ///
    /// ```ignore
    /// let x = Pexels::new("YOUR_API_KEY".to_string());
    /// x.get_photo("417173".to_string());
    /// ```
    pub fn get_photo(&self, id: String) -> ::serde_json::Value {
        self.get(&format!("{}{}", "v1/photos/", id), None)
    }
}
