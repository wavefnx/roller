use crate::Network;
use eventsource_client::{self as es, Client as EventSourceClient, SSE};
use futures::Stream;
use std::{collections::HashMap, pin::Pin};

/// The client provides methods for retrieving network metadata and establishing
/// a connection to a Server-Sent Events (SSE) stream for receiving real-time updates.
pub struct Client {
    url: String,
}

/// The response we're expecting from the EventSourceClient after connection.
/// This is a stream of Server-Sent Events (SSE) or an error.
type StreamResponse = Pin<Box<(dyn Stream<Item = Result<SSE, es::Error>> + Send + Sync + 'static)>>;

impl Client {
    /// Creates a new instance of the client with the specified URL.
    ///
    /// ### Arguments
    /// * `url` - The base URL of the conduit.xyz API.
    ///
    /// ### Returns
    /// A new instance of the `Client`.
    pub fn new<T: Into<String>>(url: T) -> Self {
        Self { url: url.into() }
    }

    /// Retrieves the network metadata from the API, without the Data field, which will be populated 
    /// through SSE (server-side event) data after initialization.
    ///
    /// This method sends a GET request to the `/networkMetadata` endpoint
    /// of the API. The response is expected to be a JSON object with network IDs as keys.
    ///
    /// ### Returns
    /// A vector of `Network` structs representing the retrieved network metadata,
    /// or an error if the request fails or the response cannot be parsed.
    pub async fn get_networks(&self) -> Result<Vec<Network>, Box<dyn std::error::Error>> {
        // Construct the endpoint URL by appending `/networkMetadata` to the base URL.
        let endpoint = format!("{}/networkMetadata", self.url);
        let response = reqwest::get(&endpoint).await?;
        // Collect the body bytes into a vector
        let body = response.bytes().await?.to_vec();
        // We're expecting the response to be a JSON object with network IDs as keys
        let networks: HashMap<String, Network> = serde_json::from_slice(&body)?;
        // The keys/IDs are the same as the `name` field of the Network,
        // therefor we can discard them.
        Ok(networks.values().cloned().collect())
    }

    /// Establishes a connection to the Server-Sent Events (SSE) stream of the rollup API.
    ///
    /// This will create an `EventSourceClient` using the `/sse` endpoint of the API
    /// and return a `Stream` of `SSE` events.
    ///
    /// ### Returns
    /// A `StreamResponse` representing the SSE stream, or an error if the connection fails.
    pub async fn get_stream(&self) -> Result<StreamResponse, es::Error> {
        Ok(es::ClientBuilder::for_url(&format!("{}/sse", self.url))?
            .build()
            .stream())
    }
}
