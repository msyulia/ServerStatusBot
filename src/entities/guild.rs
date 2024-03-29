use crate::entities::{Address, BotSettings};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub channel_id: Option<String>,
    pub addresses: Vec<Address>,
    pub settings: BotSettings,
}

impl Guild {
    pub fn new(id: String, name: String) -> Self {
        Guild {
            id,
            name,
            channel_id: None,
            addresses: vec![],
            settings: BotSettings::default(),
        }
    }

    pub fn set_channel(&mut self, channel: String) {
        self.channel_id = Some(channel);
    }

    pub fn add_address(&mut self, address: Address) {
        self.addresses.push(address);
    }

    pub fn remove_address(&mut self, address: Address) {
        self.addresses.retain(|item| *item != address);
    }
}
