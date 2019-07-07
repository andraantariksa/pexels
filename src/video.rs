impl super::Pexels {
    /// # Arguments
    ///
    /// * query - Get videos related to this `query`.
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
    ///     pexels_api_client.video_search("mountains".to_string(), 15, 1);
    /// }
    /// ```
    ///
    /// # Example Output
    ///
    /// ```
    /// Object({
    ///     "page":Number(1),
    ///     "per_page":Number(15),
    ///     "total_results":Number(367),
    ///     "url":    String("http://api-videos.pexels.com/search/mountain"),
    ///     "videos":Array(    [
    ///         Object(        {
    ///             "duration":Number(7),
    ///             "full_res":Null,
    ///             "height":Number(720),
    ///             "id":Number(857195),
    ///             "image":            String("https://images.pexels.com/videos/857195/free-video-857195.jpg?fit=crop&w=1200&h=630&auto=compress&cs=tinysrgb"),
    ///             "url":            String("https://videos.pexels.com/videos/time-lapse-video-of-night-sky-857195"),
    ///             "video_files":Array(            [
    ///                 Object(                {
    ///                     "file_type":String("video/mp4"),
    ///                     "height":Number(360),
    ///                     "id":Number(9015),
    ///                     "link":                    String("https://player.vimeo.com/external/189545487.sd.mp4?s=8cd2af1ec08f7ce121a5a6a09c78c05237943524&profile_id=164&oauth2_token_id=57447761"),
    ///                     "quality":String("sd"),
    ///                     "width":Number(640)
    ///                 }                ),
    ///                 Object(                {
    ///                     "file_type":String("video/mp4"),
    ///                     "height":Number(720),
    ///                     "id":Number(9016),
    ///                     "link":                    String("https://player.vimeo.com/external/189545487.hd.mp4?s=131fa753bd9d7d6085af29f3603f4b65cbb3ab31&profile_id=174&oauth2_token_id=57447761"),
    ///                     "quality":String("hd"),
    ///                     "width":Number(1280)
    ///                 }                ),
    ///                 Object(                {
    ///                     "file_type":String("video/mp4"),
    ///                     "height":Number(540),
    ///                     "id":Number(9017),
    ///                     "link":                    String("https://player.vimeo.com/external/189545487.sd.mp4?s=8cd2af1ec08f7ce121a5a6a09c78c05237943524&profile_id=165&oauth2_token_id=57447761"),
    ///                     "quality":String("sd"),
    ///                     "width":Number(960)
    ///                 }                ),
    ///                 Object(                {
    ///                     "file_type":String("video/mp4"),
    ///                     "height":Null,
    ///                     "id":Number(9018),
    ///                     "link":                    String("https://player.vimeo.com/external/189545487.m3u8?s=d35298d7880d7056964e7738f358f60202f75ee7&oauth2_token_id=57447761"),
    ///                     "quality":String("hls"),
    ///                     "width":Null
    ///                 }                )
    ///             ]            ),
    ///             "video_pictures":Array(            [
    ///                 Object(                {
    ///                     "id":Number(12841),
    ///                     "nr":Number(0),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-0.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12842),
    ///                     "nr":Number(1),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-1.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12843),
    ///                     "nr":Number(2),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-2.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12844),
    ///                     "nr":Number(3),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-3.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12845),
    ///                     "nr":Number(4),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-4.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12846),
    ///                     "nr":Number(5),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-5.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12847),
    ///                     "nr":Number(6),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-6.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12848),
    ///                     "nr":Number(7),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-7.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12849),
    ///                     "nr":Number(8),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-8.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12850),
    ///                     "nr":Number(9),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-9.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12851),
    ///                     "nr":Number(10),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-10.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12852),
    ///                     "nr":Number(11),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-11.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12853),
    ///                     "nr":Number(12),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-12.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12854),
    ///                     "nr":Number(13),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-13.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(12855),
    ///                     "nr":Number(14),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/857195/pictures/preview-14.jpg")
    ///                 }                )
    ///             ]            ),
    ///             "width":Number(1280)
    ///         }        ),
    ///         .....
    ///     ]    )
    /// })
    /// ```
    pub fn video_search(&self, query: String, per_page: u32, page: u32) -> ::serde_json::Value {
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
    /// ```rust
    /// extern crate pexels;
    ///
    /// fn main() {
    ///     let pexels_api_client = pexels::Pexels::new("YOUR_API_KEY".to_string());
    ///     pexels_api_client.video_popular(15, 1);
    /// }
    /// ```
    ///
    /// # Example Output
    ///
    /// ```
    /// Object({
    ///     "page":Number(1),
    ///     "per_page":Number(15),
    ///     "total_results":Number(7621),
    ///     "url":    String("http://api-videos.pexels.com/popular-videos"),
    ///     "videos":Array(    [
    ///         Object(        {
    ///             "duration":Number(18),
    ///             "full_res":Null,
    ///             "height":Number(1080),
    ///             "id":Number(2268807),
    ///             "image":            String("https://images.pexels.com/videos/2268807/free-video-2268807.jpg?fit=crop&w=1200&h=630&auto=compress&cs=tinysrgb"),
    ///             "url":            String("https://videos.pexels.com/videos/an-open-book-2268807"),
    ///             "video_files":Array(            [
    ///                 Object(                {
    ///                     "file_type":String("video/mp4"),
    ///                     "height":Number(240),
    ///                     "id":Number(100777),
    ///                     "link":                    String("https://player.vimeo.com/external/334034343.sd.mp4?s=86f5d7798a820e355dea70d0c50d892298f976af&profile_id=139&oauth2_token_id=57447761"),
    ///                     "quality":String("sd"),
    ///                     "width":Number(426)
    ///                 }                ),
    ///                 Object(                {
    ///                     "file_type":String("video/mp4"),
    ///                     "height":Number(720),
    ///                     "id":Number(100778),
    ///                     "link":                    String("https://player.vimeo.com/external/334034343.hd.mp4?s=b00f209591054430d09b2ecffa47e320a0477941&profile_id=174&oauth2_token_id=57447761"),
    ///                     "quality":String("hd"),
    ///                     "width":Number(1280)
    ///                 }                ),
    ///                 Object(                {
    ///                     "file_type":String("video/mp4"),
    ///                     "height":Number(360),
    ///                     "id":Number(100779),
    ///                     "link":                    String("https://player.vimeo.com/external/334034343.sd.mp4?s=86f5d7798a820e355dea70d0c50d892298f976af&profile_id=164&oauth2_token_id=57447761"),
    ///                     "quality":String("sd"),
    ///                     "width":Number(640)
    ///                 }                ),
    ///                 Object(                {
    ///                     "file_type":String("video/mp4"),
    ///                     "height":Number(1080),
    ///                     "id":Number(100780),
    ///                     "link":                    String("https://player.vimeo.com/external/334034343.hd.mp4?s=b00f209591054430d09b2ecffa47e320a0477941&profile_id=175&oauth2_token_id=57447761"),
    ///                     "quality":String("hd"),
    ///                     "width":Number(1920)
    ///                 }                ),
    ///                 Object(                {
    ///                     "file_type":String("video/mp4"),
    ///                     "height":Number(540),
    ///                     "id":Number(100781),
    ///                     "link":                    String("https://player.vimeo.com/external/334034343.sd.mp4?s=86f5d7798a820e355dea70d0c50d892298f976af&profile_id=165&oauth2_token_id=57447761"),
    ///                     "quality":String("sd"),
    ///                     "width":Number(960)
    ///                 }                ),
    ///                 Object(                {
    ///                     "file_type":String("video/mp4"),
    ///                     "height":Null,
    ///                     "id":Number(100782),
    ///                     "link":                    String("https://player.vimeo.com/external/334034343.m3u8?s=f4a110cb30a7d0f9894b034f8d75e6a8d4fe0348&oauth2_token_id=57447761"),
    ///                     "quality":String("hls"),
    ///                     "width":Null
    ///                 }                )
    ///             ]            ),
    ///             "video_pictures":Array(            [
    ///                 Object(                {
    ///                     "id":Number(252273),
    ///                     "nr":Number(0),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-0.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252274),
    ///                     "nr":Number(1),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-1.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252275),
    ///                     "nr":Number(2),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-2.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252276),
    ///                     "nr":Number(3),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-3.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252277),
    ///                     "nr":Number(4),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-4.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252278),
    ///                     "nr":Number(5),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-5.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252279),
    ///                     "nr":Number(6),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-6.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252280),
    ///                     "nr":Number(7),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-7.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252281),
    ///                     "nr":Number(8),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-8.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252282),
    ///                     "nr":Number(9),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-9.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252283),
    ///                     "nr":Number(10),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-10.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252284),
    ///                     "nr":Number(11),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-11.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252285),
    ///                     "nr":Number(12),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-12.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252287),
    ///                     "nr":Number(13),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-13.jpg")
    ///                 }                ),
    ///                 Object(                {
    ///                     "id":Number(252288),
    ///                     "nr":Number(14),
    ///                     "picture":                    String("https://static-videos.pexels.com/videos/2268807/pictures/preview-14.jpg")
    ///                 }                )
    ///             ]            ),
    ///             "width":Number(1920)
    ///         }        ),
    ///         .....
    ///     ]    )
    /// })
    /// ```
    pub fn video_popular(&self, per_page: u32, page: u32) -> ::serde_json::Value {
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
