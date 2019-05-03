impl super::Pexels {
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
