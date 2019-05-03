extern crate reqwest;
extern crate serde_json;

const API_URL: &'static str = "https://api.pexels.com/";

#[derive(Clone, Debug)]
struct Pexels {
    api_key: String,
    reqwest_client: reqwest::Client,
}

impl Pexels {
    fn new(api_key: String) -> Pexels {
        Pexels {
            api_key,
            reqwest_client: reqwest::Client::new(),
        }
    }

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

pub mod photo;
pub mod video;
