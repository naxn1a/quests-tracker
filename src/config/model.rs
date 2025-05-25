#[derive(Debug, Clone)]
pub struct DotEnvConfig {
    pub server: Server,
    pub database: Database,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub limit: u64,
    pub timeout: u64,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct AdventurersSecret {
    pub secret: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct GuildCommandersSecret {
    pub secret: String,
    pub refresh_token: String,
}
