//! # Wave
//!  
//! Use [Wave API](https://wireway.ch) Wave API to fetch music by search query
 
use reqwest::blocking::{Client, Response as ReqwestResponse};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Represents a music item retrieved from the Wave API.
#[derive(Serialize, Deserialize, Debug)]
pub struct WaveMusic {
    /// The title of the music item.
    title: Option<String>,
    /// The name of the uploader of the music item.
    #[serde(rename = "uploaderName")]
    uploader_name: Option<String>,
    /// The URL of the uploader's profile.
    #[serde(rename = "uploaderUrl")]
    uploader_url: Option<String>,
    /// The duration of the music item in seconds.
    duration: Option<u32>,
    /// The unique identifier of the music item.
    id: Option<String>,
}

/// Represents the response structure from the Wave API.
#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    /// A list of music items.
    items: Vec<WaveMusic>,
}

impl fmt::Display for WaveMusic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} from {}",
            self.title.clone().unwrap_or_default(),
            self.uploader_name.clone().unwrap_or_default()
        )
    }
}

impl WaveMusic {
    /// Creates a new WaveMusic instance by querying the Wave API with the specified search term.
    ///
    /// # Arguments
    ///
    /// * `q` - A string slice that holds the search term.
    ///
    /// # Example
    ///
    /// ```
    /// let music_items = WaveMusic::new("example search term".to_string());
    /// for item in music_items.unwrap() {
    ///     println!("{}", item);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the HTTP request fails or if the response cannot be parsed.
    pub fn new(q: String) -> Result<Vec<WaveMusic>, Box<dyn Error>> {
        let url = format!("https://api.wireway.ch/wave/ytmusicsearch?q={}", q);
        let client = Client::new();
        let response = Self::fetch_data(&client, &url)?;
        let response_json: ApiResponse = Self::parse_response(response)?;
        Ok(response_json.items)
    }

    /// Fetches data from the specified URL using the given HTTP client.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the HTTP client.
    /// * `url` - A string slice that holds the URL to fetch data from.
    ///
    /// # Errors
    ///
    /// This function will return an error if the HTTP request fails or if the response status is not successful.
    fn fetch_data(client: &Client, url: &str) -> Result<ReqwestResponse, Box<dyn Error>> {
        let response = client.get(url).send()?;
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(format!("Failed to fetch data: HTTP {}", response.status()).into())
        }
    }

    /// Parses the response body into the ApiResponse structure.
    ///
    /// # Arguments
    ///
    /// * `response` - The HTTP response to parse.
    ///
    /// # Errors
    ///
    /// This function will return an error if the response body cannot be parsed as JSON.
    fn parse_response(response: ReqwestResponse) -> Result<ApiResponse, Box<dyn Error>> {
        let text = response.text()?;
        let response_json: ApiResponse = serde_json::from_str(&text).map_err(|e| {
            eprintln!("Failed to parse JSON: {}", e);
            Box::new(e) as Box<dyn Error>
        })?;
        Ok(response_json)
    }
}
