pub enum Provider {
    Card,
}

impl Provider {
    pub fn from_str(provider: &String) -> anyhow::Result<Self> {
        match provider.to_lowercase().as_str() {
            "card" => Ok(Provider::Card),
            _ => Err(anyhow::anyhow!("Unsupported Provider")),
        }
    }
}
