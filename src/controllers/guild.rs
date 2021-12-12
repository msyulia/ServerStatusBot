use crate::{
    entities::{Address, GuildId},
    errors::GuildResult,
    proxies::GuildProxy,
};

pub struct GuildController<Proxy: GuildProxy + Send + Sync> {
    proxy: Proxy,
}

impl<Proxy: GuildProxy + Send + Sync> GuildController<Proxy> {
    pub fn new(proxy: Proxy) -> Self {
        GuildController { proxy }
    }

    pub async fn server_addresses(&self, id: String) -> GuildResult<Vec<Address>> {
        self.proxy.list_addresses(GuildId::new(id)).await
    }

    pub async fn add_server(&self, guild: GuildId, address: Address) -> GuildResult<()> {
        self.proxy.add_address(guild, address).await
    }

    pub async fn create_guild(&self, guild: GuildId, name: String) -> GuildResult<()> {
        self.proxy.create_guild(guild, name).await
    }

    pub async fn delete_guild(&self, guild: GuildId) -> GuildResult<()> {
        self.proxy.delete_guild(guild).await
    }
}