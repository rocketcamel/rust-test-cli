pub enum Provider {
    Card,
}

impl Provider {
    pub fn from_str(provider: &str) -> anyhow::Result<Self> {
        match provider.to_lowercase().as_str() {
            "card" => Ok(Provider::Card),
            _ => Err(anyhow::anyhow!("Unsupported Provider")),
        }
    }
}
