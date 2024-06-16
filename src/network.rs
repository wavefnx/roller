use ratatui::{text::Text, widgets::Row};
use serde::{Deserialize, Deserializer};

/// Represents the data associated with a network.
#[derive(Debug, serde::Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    // The current block number of the network.
    pub block_number: u64,
    // The transactions per second of the network.
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub tps: f32,
    // The gas per second of the network.
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub gps: f32,
    // The data per second of the network, is processing.
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    pub dps: f32,
    //
    // The fields below are disabled until required.
    //
    // pub data_count: u64,
    // pub gas_count: u64,
    // pub timestamp: u64,
    // pub tx_count: u64,
}

#[derive(Debug, serde::Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    // The name of the network. This is also the networks ID.
    pub name: String,
    // The label/human-readable name of the network.
    pub label: String,
    #[serde(deserialize_with = "deserialize_chain_id_to_name")]
    // The parent chain, which is also used as a Settlment Layer
    // in the case of an Layer 2 network.
    pub parent_chain: String,
    // The data availability layer of the network.
    pub da: String,
    // The stack used by the network.
    pub stack: String,
    // The data associated with the network.
    // Populated through SSE (server-side event) data after initialization.
    pub data: Option<Data>,
    //
    // The fields below are disabled until required.
    //
    // pub provider: String,
    // pub website: String,
    // pub explorer: String,
}

/// Deserializes a chain ID into a chain name.
///
/// ### Arguments
/// * `deserializer` - The deserializer used to deserialize the chain ID,serde in this case.
///
/// ### Returns
/// The chain name corresponding to the deserialized chain ID.
pub fn deserialize_chain_id_to_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let chain_id: String = Deserialize::deserialize(deserializer)?;
    let chain_id: u64 = chain_id.parse().unwrap_or_default();
    Ok(Network::name_from_chain_id(chain_id))
}

/// Deserializes a string into a f32. The fields utilizing this deserialization function
/// will be Strings, since they received from an SSE event in a JSON form.
/// We're parsing it into a f32 to be able to work with the values before displaying them.
///
/// ### Arguments
/// * `deserializer` - The deserializer used to deserialize the value, serde in this case.
///
/// ### Returns
/// The f32 value of the deserialized string.
pub fn deserialize_string_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    Ok(value.parse().unwrap_or_default())
}

impl Network {
    /// A new instance of `Network` with the provided parameters.
    pub fn new(
        name: String,
        label: String,
        parent_chain: String,
        da: String,
        stack: String,
        data: Option<Data>,
    ) -> Self {
        Self {
            name,
            label,
            parent_chain,
            da,
            stack,
            data,
        }
    }

    /// Updates the data associated with the network.
    ///
    /// ### Arguments
    /// * `data` - The new data of the network.
    pub fn update_data(&mut self, data: Option<Data>) {
        self.data = data;
    }

    /// {Unstable} Converts the network into a ratatui `Row` widget.
    /// It will get ownership of the data field before returning.
    ///
    /// ### Returns
    /// A `Row` widget containing the network's data in a formatted manner.
    pub fn to_row(&self) -> Row {
        let data = self.data.to_owned().unwrap_or_default();
        Row::new(vec![
            Text::raw(&self.label),
            Text::raw(data.block_number.to_string()),
            Text::raw(data.tps.to_string()),
            Text::raw(data.gps.to_string()),
            Text::raw(data.dps.to_string()),
            Text::raw(&self.stack),
            Text::raw(&self.da),
            Text::raw(&self.parent_chain),
        ])
    }

    /// Returns the chain name based on the chain ID.
    ///
    /// ### Arguments
    /// * `id` - The chain ID.
    ///
    /// ### Returns
    /// The chain name corresponding to the chain ID, or "unknown" if the ID is not recognized or listed.
    fn name_from_chain_id(id: u64) -> String {
        match id {
            1 => String::from("ethereum"),
            8453 => String::from("base"),
            42161 => String::from("arbitrum"),
            _ => String::from("unknown"),
        }
    }
}
