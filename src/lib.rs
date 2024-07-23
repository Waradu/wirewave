//! # WireWave
//!  
//! Use the [Wave API](https://wireway.ch) to fetch music by search query and retrieve thumbnails.

use reqwest::blocking::{Client, Response as ReqwestResponse};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Represents a music item retrieved from the Wave API.
#[derive(Serialize, Deserialize, Debug)]
pub struct WaveMusic {
    /// The title of the music item.
    pub title: Option<String>,
    /// The name of the uploader of the music item.
    #[serde(rename = "uploaderName")]
    pub uploader_name: Option<String>,
    /// The URL of the uploader's profile.
    #[serde(rename = "uploaderUrl")]
    pub uploader_url: Option<String>,
    /// The duration of the music item in seconds.
    pub duration: Option<u32>,
    /// The unique identifier of the music item.
    pub id: Option<String>,
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
    /// let music_items = WaveMusic::new("example search term".to_string()).unwrap();
    /// for item in music_items {
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

    /// Fetches the thumbnail image data for the music item.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::copy;
    /// use std::path::Path;
    /// use wirewave::*;
    ///
    /// let music_items = WaveMusic::new("example search term".to_string()).unwrap();
    /// if let Some(item) = music_items.first() {
    ///     let mut thumbnail_response = item.thumbnail().unwrap();
    ///     let name = format!("{}.png", item.id.as_ref().unwrap());
    ///     let path = Path::new(&name);
    ///     let mut file = File::create(&path).unwrap();
    ///     copy(&mut thumbnail_response, &mut file).unwrap();
    ///     println!("Image fetched and saved to {:?}", path);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the music item does not have an ID or if the HTTP request fails.
    pub fn thumbnail(&self) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
        // Ensure the music item has an ID
        let id = self.id.as_ref().ok_or("Music item does not have an ID")?;
        // Construct the thumbnail URL
        let url = format!("https://api.wireway.ch/wave/thumbnail/{}", id);

        let response = reqwest::blocking::get(&url)?;

        // Check for response errors
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(format!("Failed to fetch thumbnail: HTTP {}", response.status()).into())
        }
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
        let response_json: ApiResponse = serde_json::from_str(&text)?;
        Ok(response_json)
    }
}
