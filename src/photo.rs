impl super::Pexels {
    /// # Arguments
    ///
    /// * query - Search photos related to the `query`
    /// * per_page - Defines the number of results per page (max: 80)
    /// * page - Defines the number of the page
    ///
    /// # Example
    ///
    /// ```rust
    /// extern crate pexels;
    ///
    /// fn main() {
    ///     let pexels_api_client = pexels::Pexels::new("YOUR_API_KEY".to_string());
    ///     pexels_api_client.photo_search("mountains".to_string(), 15, 1);
    /// }
    /// ```
    ///
    /// # Example Output
    ///
    /// ```
    /// Object({
    ///     "next_page":    String("https://api.pexels.com/v1/search/?page=2&per_page=15&query=mountain"),
    ///     "page":Number(1),
    ///     "per_page":Number(15),
    ///     "photos":Array(    [
    ///         Object(        {
    ///             "height":Number(3248),
    ///             "id":Number(417173),
    ///             "photographer":String("Pixabay"),
    ///             "photographer_url":            String("https://www.pexels.com/@pixabay"),
    ///             "src":Object(            {
    ///                 "landscape":                String("https://images.pexels.com/photos/417173/pexels-photo-417173.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200"),
    ///                 "large":                String("https://images.pexels.com/photos/417173/pexels-photo-417173.jpeg?auto=compress&cs=tinysrgb&h=650&w=940"),
    ///                 "large2x":                String("https://images.pexels.com/photos/417173/pexels-photo-417173.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940"),
    ///                 "medium":                String("https://images.pexels.com/photos/417173/pexels-photo-417173.jpeg?auto=compress&cs=tinysrgb&h=350"),
    ///                 "original":                String("https://images.pexels.com/photos/417173/pexels-photo-417173.jpeg"),
    ///                 "portrait":                String("https://images.pexels.com/photos/417173/pexels-photo-417173.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800"),
    ///                 "small":                String("https://images.pexels.com/photos/417173/pexels-photo-417173.jpeg?auto=compress&cs=tinysrgb&h=130"),
    ///                 "tiny":                String("https://images.pexels.com/photos/417173/pexels-photo-417173.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280")
    ///             }            ),
    ///             "url":            String("https://www.pexels.com/photo/altitude-clouds-cold-daylight-417173/"),
    ///             "width":Number(4872)
    ///         }        ),
    ///         .....
    ///     ]    ),
    ///     "total_results":Number(9478)
    /// })
    /// ```
    pub fn photo_search(&self, query: String, per_page: u32, page: u32) -> ::serde_json::Value {
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
    /// ```rust
    /// extern crate pexels;
    ///
    /// fn main() {
    ///     let pexels_api_client = pexels::Pexels::new("YOUR_API_KEY".to_string());
    ///     pexels_api_client.photo_curated(15, 1);
    /// }
    /// ```
    ///
    /// # Example Output
    ///
    /// ```
    /// Object({
    ///     "next_page":    String("https://api.pexels.com/v1/curated/?page=2&per_page=15"),
    ///     "page":Number(1),
    ///     "per_page":Number(15),
    ///     "photos":Array(    [
    ///         Object(        {
    ///             "height":Number(2667),
    ///             "id":Number(1492232),
    ///             "photographer":String("Soloman Soh"),
    ///             "photographer_url":            String("https://www.pexels.com/@soloman-soh-674993"),
    ///             "src":Object(            {
    ///                 "landscape":                String("https://images.pexels.com/photos/1492232/pexels-photo-1492232.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200"),
    ///                 "large":                String("https://images.pexels.com/photos/1492232/pexels-photo-1492232.jpeg?auto=compress&cs=tinysrgb&h=650&w=940"),
    ///                 "large2x":                String("https://images.pexels.com/photos/1492232/pexels-photo-1492232.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940"),
    ///                 "medium":                String("https://images.pexels.com/photos/1492232/pexels-photo-1492232.jpeg?auto=compress&cs=tinysrgb&h=350"),
    ///                 "original":                String("https://images.pexels.com/photos/1492232/pexels-photo-1492232.jpeg"),
    ///                 "portrait":                String("https://images.pexels.com/photos/1492232/pexels-photo-1492232.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800"),
    ///                 "small":                String("https://images.pexels.com/photos/1492232/pexels-photo-1492232.jpeg?auto=compress&cs=tinysrgb&h=130"),
    ///                 "tiny":                String("https://images.pexels.com/photos/1492232/pexels-photo-1492232.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280")
    ///             }            ),
    ///             "url":            String("https://www.pexels.com/photo/architectural-photography-of-glass-buliding-1492232/"),
    ///             "width":Number(4000)
    ///         }        ),
    ///         .....
    ///     ]    )
    /// })
    /// ```
    pub fn photo_curated(&self, per_page: u32, page: u32) -> ::serde_json::Value {
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
    /// ```rust
    /// extern crate pexels;
    ///
    /// fn main() {
    ///     let pexels_api_client = pexels::Pexels::new("YOUR_API_KEY".to_string());
    ///     pexels_api_client.photo_detail("417173".to_string());
    /// }
    /// ```
    ///
    /// # Example Output
    ///
    /// ```
    /// Object({
    ///     "height":Number(4160),
    ///     "id":Number(2608372),
    ///     "photographer":String("Kei Scampa"),
    ///     "photographer_url":    String("https://www.pexels.com/@kei-scampa-1201427"),
    ///     "src":Object(    {
    ///         "landscape":        String("https://images.pexels.com/photos/2608372/pexels-photo-2608372.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200"),
    ///         "large":        String("https://images.pexels.com/photos/2608372/pexels-photo-2608372.jpeg?auto=compress&cs=tinysrgb&h=650&w=940"),
    ///         "large2x":        String("https://images.pexels.com/photos/2608372/pexels-photo-2608372.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940"),
    ///         "medium":        String("https://images.pexels.com/photos/2608372/pexels-photo-2608372.jpeg?auto=compress&cs=tinysrgb&h=350"),
    ///         "original":        String("https://images.pexels.com/photos/2608372/pexels-photo-2608372.jpeg"),
    ///         "portrait":        String("https://images.pexels.com/photos/2608372/pexels-photo-2608372.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800"),
    ///         "small":        String("https://images.pexels.com/photos/2608372/pexels-photo-2608372.jpeg?auto=compress&cs=tinysrgb&h=130"),
    ///         "tiny":        String("https://images.pexels.com/photos/2608372/pexels-photo-2608372.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280")
    ///     }    ),
    ///     "url":    String("https://www.pexels.com/photo/woman-inside-vehicle-2608372/"),
    ///     "width":Number(6240)
    /// })
    /// ```
    pub fn photo_detail(&self, id: String) -> ::serde_json::Value {
        self.get(&format!("{}{}", "v1/photos/", id), None)
    }
}
