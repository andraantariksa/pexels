impl super::Pexels {
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

    pub fn get_photo(&self, id: String) -> ::serde_json::Value {
        self.get(&format!("{}{}", "v1/photos/", id), None)
    }
}
