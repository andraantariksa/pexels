/*!
The `pexels` crate provides API wrapper for Pexels. It based on [Pexels API Documentation](https://www.pexels.com/api/documentation/).

To get the API key, you have request the access from [Request API Access – Pexels](https://www.pexels.com/api/new/).

This library are depends on [serde-json](https://github.com/serde-rs/json) crate to handle the result, thus you have to read the documentation [serde_json - Rust](https://docs.serde.rs/serde_json/index.html), especially [serde_json::Value - Rust](https://docs.serde.rs/serde_json/enum.Value.html).

# Setup

Add this line to your `Cargo.toml` file, below `[dependencies]`

```toml
pexels = "*"
```

and this to your crate root file e.g `main.rs`:

```rust
extern crate pexels;
```

Done! Now you can use this API wrapper.

# Example

This example shows how to get the list of *mountains* photos.

```rust
extern crate pexels;

fn main() {
    let pexels_api_client = pexels::Pexels::new("YOUR_API_KEY".to_owned());
    pexels_api_client.photo_search("mountains".to_string(), 15, 1);
}
```

and you can run it using `cargo run` ! Simply as that.

# Random photo

If you want to get a random photo, you can use the `curated_photo` function and set per_page to 1 and page to a random number between 1 and 1000 to get a beautiful random photo. You can do the same with popular searches if you want to get a random photo with a specific topic.

# Image formats

* original - The size of the original image is given with the attributes width and height.
*large - This image has a maximum width of 940px and a maximum height of 650px. It has the aspect ratio of the original image.
* large2x - This image has a maximum width of 1880px and a maximum height of 1300px. It has the aspect ratio of the original image.
* medium - This image has a height of 350px and a flexible width. It has the aspect ratio of the original image.
* small - This image has a height of 130px and a flexible width. It has the aspect ratio of the original image.
* portrait    This image has a width of 800px and a height of 1200px.
* landscape - This image has a width of 1200px and height of 627px.
* tiny - This image has a width of 280px and height of 200px.
*/
extern crate reqwest;
extern crate serde_json;

const API_URL: &'static str = "https://api.pexels.com/";

#[derive(Clone, Debug)]
pub struct Pexels {
    api_key: String,
    reqwest_client: reqwest::Client,
}

impl Pexels {
    /// Create a new Pexels API client.
    pub fn new(api_key: String) -> Pexels {
        Pexels {
            api_key,
            reqwest_client: reqwest::Client::new(),
        }
    }

    /// Request builder and gateway with [reqwest](https://crates.io/crates/reqwest)
    fn get(&self, endpoint: &str, param: Option<Vec<(&str, String)>>) -> serde_json::Value {
        let mut request_builder = self.reqwest_client.get(&(API_URL.to_owned() + endpoint));
        request_builder = match param {
            Some(x) => request_builder.query(&x),
            None => request_builder,
        };
        let mut request = request_builder
            .header(reqwest::header::AUTHORIZATION, self.api_key.clone())
            .send()
            .expect(&format!("Failed to send request to {}", endpoint));
        serde_json::from_str(&request.text().unwrap()).expect("Failed to read the response")
    }
}

/// Photo related API's implementation
pub mod photo;
/// Video related API's implementation
pub mod video;
